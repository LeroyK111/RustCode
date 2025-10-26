# 给 NdArray 装上 CUDA 的轮子

Ndarry是Rust编程语言中的一个高性能多维、多类型数组库。它提供了类似 numpy 的多种多维数组的算子。与 Python 相比 Rust 生态缺乏类似 CuPy, Jax 这样利用CUDA 进行加速的开源项目。虽然 Hugging Face 开源的 candle 可以使用 CUDA backend 但是 candle 项瞄准的是大模型的相关应用。本着自己造轮子是最好的学习方法，加上受到 Karpathy llm.c 项目的感召（这个项目是学习如何编写 CUDA kernel 的最好参考之一），我搞了一个 rlib 库给 NdArray 加上一个跑在 CUDA 上的矩阵乘法。ndarray-linalg 库提供的点乘其中一个实现（features）是依赖 openblas 的，对于低维的矩阵性能可以满足需求，但是机器学习，深度学习这些领域遇到的矩阵动辄上千维，openblas 里古老的优化到极致的 Fortran 代码还是敌不过通过并行性开挂的CUDA。

动手之前我参考了 Karpathy 写的 matmul_forward.cu 文件，Karpathy 在里面实现了3种矩阵乘法，纯C代码的 kernel 函数性能明显不如调用CuBlas库和CuBlasLT库的。cublas 库是由NVIDIA提供的一个用于GPU上执行基本线性代数子程序（BLAS）操作的软件库。它是CUDA工具包的一部分，专门针对NVIDIA图形处理单元（GPUs）上的高性能科学计算优化。用 cublas 库另外一个好处就是不需要去通过 blockDim，threadIdx 去计算数据的下标。所以我决定用 cublas 来实现，虽然性能不如 CuBlasLT 但是接口要简单很多。

如何在 RUST 中调用 CUDA?

首先是一个好消息，现在 github 上已经有一个 rust 库 cudarc 封装了 CUDA 的大部分 API 当然包括cublas，甚至它还提供可以把文本编译成 PTX （NVIDIA 的官方解释: a low-levelparallel thread executionvirtual machine and instruction set architecture (ISA). PTX exposes the GPU as a data-parallel computingdevice.）的宏。如果只是直接调用 cudarc 封装好的接口，明显不符合通过造轮子解释如何通过 RUST 调用 CUDA 的目的。这里只好采用和 candle 一样的方案，利用 bindgen_cuda 库将 rust 项目 src 下面包括子目录里面的 .cu 文件全部编译打包成一个静态库，然后通过 FFI 编译连接到 rust 里面。

```
接下来介绍一下使用 bindgen_cuda 的步骤：
[build-dependencies]
bindgen_cuda = "0.1.5"
在项目根目录下面添加 build.rs 文件，加入以下内容:
use std::env;

fn main() {
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let builder = bindgen_cuda::Builder::default();
    builder.build_lib("libcuda.a"); // 将所有.cu 编译打包成一个静态库
    println!("cargo:rustc-link-search={}", dir); //增加查找库文件libcuda.a的路径
    println!("cargo:rustc-link-search={}", "/usr/local/cuda/lib64");//增加查找cuda库的路径
    println!("cargo:rustc-link-lib=static=cuda");//给 ld 增加一个连接参数 -lcuda 
    println!("cargo:rustc-link-lib=cudart");//连接cuda 运行时 libcudart.so 
    println!("cargo:rustc-link-lib=cublas");//连接cublas
    println!("cargo:rustc-link-lib=stdc++");// .cu 其实是C++, NVCC会调用g++进行编译，所以需要C++标准库
    println!("cargo:rustc-link-lib=cblas");// 这是为了测试 ndarray-linalg 的 dot 函数
}
bindgen_cuda 相关的配置和代码完成。接下来就是编译.cu 文件来封装 cublas 提供的矩阵乘法函数cublasSgemm。

cublasSgemm函数的定义如下：

cublasStatus_t cublasSgemm(cublasHandle_t handle,
                           cublasOperation_t transa, cublasOperation_t transb,
                           int m, int n, int k,
                           const float *alpha,
                           const float *A, int lda,
                           const float *B, int ldb,
                           const float *beta,
                           float *C, int ldc)
实际上这个函数执行的是 C = alpha * A * B + beta * C，这里只需要进行矩阵乘法所以 alpha = 1.0_f32 beta=0.0_f32。其他参数的说明如下：

handle 是一个结构体的指针，用 cublasCreate(&cublas_handle)这样的方式来创建；
transa 和 transb 表示A,B矩阵是否需要进行转置，NdArray 是行优先的cublas需要列优先，所以A,B都需要转置取值为CUBLAS_OP_T表示要转置，而CUBLAS_OP_N表示不转;
m 是矩阵 A 的行数；
n 是矩阵 B 的列；
k 是矩阵A的列数和矩阵B的行数；
A 矩阵A的指针；
lda A矩阵的前导维度，由于数据在内存里面是连续存储的，ldb表示列优先访问数据步长所以是A的列数（倒置后的行数）;
ldb B矩阵的前导维度，这里取B的行数（倒置后的列数）;
具体封装的代码在 matmul.cu 中如下：

// 使用C语言接口声明一个矩阵乘法函数，这允许这个函数可以被其他C程序调用。
extern "C" void matmul_cublas(float *out,
                              const float *a, const float *b,
                              int m, int n, int k)
{
    // 定义标量alpha和beta，这在矩阵乘法中作为系数使用：C = alpha*A*B + beta*C。
    const float alpha = 1.0f;
    const float beta = 0.0f;

    // 声明指向GPU内存中矩阵的指针。
    float *a_mat, *b_mat, *out_mat;

    // 在GPU上分配内存空间，为矩阵A和B以及输出矩阵out_mat。
    cudaCheck(cudaMalloc(&a_mat, m * k * sizeof(float)));
    cudaCheck(cudaMalloc(&b_mat, n * k * sizeof(float)));
    cudaCheck(cudaMalloc(&out_mat, m * n * sizeof(float)));

    // 将矩阵A和B的数据从主机复制到分配的GPU内存。
    cudaCheck(cudaMemcpy(a_mat, a, m * k * sizeof(float), cudaMemcpyHostToDevice));
    cudaCheck(cudaMemcpy(b_mat, b, n * k * sizeof(float), cudaMemcpyHostToDevice));

    // 调用cuBLAS库函数cublasSgemm执行单精度的矩阵乘法。
    // 注意：CUBLAS_OP_T表示传递给cuBLAS的矩阵在GPU中是转置的。
    cublasCheck(cublasSgemm(cublas_handle, CUBLAS_OP_T, CUBLAS_OP_T, 
                            m, n, k, &alpha, 
                            a_mat, k, 
                            b_mat, n, 
                            &beta, out_mat, m));

    // 将结果从GPU内存复制回主机内存。
    cudaCheck(cudaMemcpy(out, out_mat, m * n * sizeof(float), cudaMemcpyDeviceToHost));

    // 清理，在GPU上分配的内存空间。
    if (a_mat)
        cudaCheck(cudaFree(a_mat));
    if (b_mat)
        cudaCheck(cudaFree(b_mat));
    if (out_mat)
        cudaCheck(cudaFree(out_mat));
}
在这段代码中，使用了一些辅助宏cudaCheck和cublasCheck来检查 CUDA 和 cuBLAS 调用是否成功。这些宏来自 Karpathy llm.c 项目的 common.h。

对应的 RUST 的代码如下：

// Extern block defining functions implemented in foreign code (e.g. C/C++ using CUDA).
extern "C" {
    fn matmul_cublas(
        out: *mut c_float,
        a: *const c_float,
        b: *const c_float,
        m: size_t,
        n: size_t,
        k: size_t,
    );
    fn _init_cublas();
    fn _destory_cublas();
}

// Function to perform matrix multiplication using the cuBLAS library.
pub fn matmul<D1: Dimension, D2: Dimension, D3: Dimension>(
    out: &mut ArrayBase<OwnedRepr<f32>, D1>,
    a: &ArrayBase<ViewRepr<&f32>, D2>,
    b: &ArrayBase<ViewRepr<&f32>, D3>,
) {
    let out_ptr = out.as_mut_ptr();
    let a_ptr = a.as_ptr();
    let b_ptr = b.as_ptr();

    let (m, n, k) = get_shape(a, b);
    unsafe {
        _init_cublas();
        matmul_cublas(out_ptr, a_ptr, b_ptr, m, n, k); // Calling the foreign CUDA function.
        _destory_cublas();
    }
}
其中 _init_cublas() 和 _destory_cublas() 分别用于调用 cublasCreate 和 cublasDestroy 。我把handle 实现成了 singleton，还加上了一个计数器防止多次 free() 导致的内存错误。

接下来通过定义一个 trait 来给 NdArray 数组加上 cuda_dot 的方法。

// A trait that defines a CUDA-based dot product between arrays.
pub trait CudaDot<Rhs> {
    type Output;
    // The method signature for performing the dot product using CUDA.
    fn cuda_dot(&self, rhs: &Rhs) -> Self::Output;
}

// Implementation of CudaDot for 1D owned representation arrays.
impl CudaDot<ArrayBase<OwnedRepr<f32>, Ix1>> for ArrayBase<OwnedRepr<f32>, Ix1> {
    type Output = ArrayBase<OwnedRepr<f32>, Ix1>;

    // Performs dot product on 1D arrays using CUDA and returns the result as a 1-element array.
    fn cuda_dot(&self, rhs: &ArrayBase<OwnedRepr<f32>, Ix1>) -> Self::Output {
        let mut out = Array::from_elem(1, 0.0_f32);
        matmul(&mut out, &self.view(), &rhs.t());
        return out;
    }
}

// Implementation of CudaDot for multiplying a 1D array with a 2D array.
impl CudaDot<ArrayBase<OwnedRepr<f32>, Ix2>> for ArrayBase<OwnedRepr<f32>, Ix1> {
    type Output = ArrayBase<OwnedRepr<f32>, Ix1>;

    // Performs multiplication of a 1D array with a 2D array.
    fn cuda_dot(&self, rhs: &ArrayBase<OwnedRepr<f32>, Ix2>) -> Self::Output {
        let mut out = Array::from_elem(1, 0.0_f32);
        matmul(&mut out, &self.view(), &rhs.view());
        return out;
    }
}

// Implementation of CudaDot for multiplying a 2D array with a 1D array.
impl CudaDot<ArrayBase<OwnedRepr<f32>, Ix1>> for ArrayBase<OwnedRepr<f32>, Ix2> {
    type Output = ArrayBase<OwnedRepr<f32>, Ix1>;

    // Performs multiplication of a 2D array with a 1D array.
    fn cuda_dot(&self, rhs: &ArrayBase<OwnedRepr<f32>, Ix1>) -> Self::Output {
        let mut out = Array::from_elem(1, 0.0_f32);
        matmul(&mut out, &rhs.view(), &self.view());
        return out;
    }
}

// Implementation of CudaDot for 2D arrays.
impl CudaDot<ArrayBase<OwnedRepr<f32>, Ix2>> for ArrayBase<OwnedRepr<f32>, Ix2> {
    type Output = ArrayBase<OwnedRepr<f32>, Ix2>;

    // Performs dot product on two 2D arrays using CUDA and returns a new 2D array.
    fn cuda_dot(&self, rhs: &ArrayBase<OwnedRepr<f32>, Ix2>) -> Self::Output {
        let (m, n, _) = get_shape(&self.view(), &rhs.view());
        let mut out = Array::from_elem((n, m), 0.0_f32);
        matmul(&mut out, &self.view(), &rhs.view());
        return out;
    }
}
这里实现了 1D, 2D 矩阵之间的点乘。trait 是 rust 非常棒的特性，无需继承，组合等等就可以给已有库增加新的功能。trait 确实是 Rust 类型系统的基石，使得代码更模块化、灵活且易于维护。

核心代码就全部介绍完了。既然是为了利用 CUDA 的异构并行计算能力，当然需要对比一下 cuda_dot 与 NdArray-linalg 库提供的 dot 的性能。

对比测试的代码如下：

    fn dot_with_ndarry() {
        let a = Array::from_elem((H_SIZE, H_SIZE), 1.0_f32);
        let b = Array::from_elem((H_SIZE, V_SIZE), 1.0_f32);
        let start = Instant::now();
        for _ in 0..100 {
            let _ = a.dot(&b);
        }
        println!("ndarray dot elapsed: {:.2?}", start.elapsed());
    }

    fn dot_with_cuda() {
        let a = Array::from_elem((H_SIZE, H_SIZE), 1.0_f32);
        let b = Array::from_elem((H_SIZE, V_SIZE), 1.0_f32);
        let start = Instant::now();
        for _ in 0..100 {
            let _ = a.cuda_dot(&b);
        }
        println!("matmul elapsed: {:.2?}", start.elapsed());
    }
在低维度情况下，NdArray-linalg 性能比 cuda_dot, 但是维度以上去cuda_dot的优势就很明显了。下面是具体测试的数据。

硬件环境：微软 Azure 提供的带一张 Nvidia A10 显卡，36 核 CPU的云主机 (感谢微软的慷慨)。

软件：

g++ (Ubuntu 9.4.0-1ubuntu1~20.04.2) 9.4.0

rustc 1.77.2 (25ef9e3d8 2024-04-09)

cuda_12.1

Nvidia Driver Version: 535.154.05

  Rows(H_SIZE)   Columns(V_SIZE)   ndarra-linalg   cuda_dot  
 -------------- ----------------- --------------- ---------- 
            64                64   2.27ms          9.89ms    
           128                80   11.37ms         10.66ms   
           768               128   438.01ms        57.86ms   
          2048              1000   22800ms         323.30ms
从测试可以看出，在高维度矩阵乘法场景下 cublas 体现出了巨大的优势。

全部的代码在：https://github.com/Lyn-liyuan/ndarray-cuda-matmul

```