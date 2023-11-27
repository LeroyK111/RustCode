/*
CPU是计算机的处理引擎，它执行指令并进行计算，使其成为性能方面最重要的组件之一。CPU由多个核心组成，每个核心都能独立执行指令。为了充分利用这些核心，编写利用并行性同时执行多个线程的代码非常重要。

假设我们有一大堆需要调整大小的图片，如果我们按顺序处理，将花费很长时间，因为每次迭代都必须等待前一个迭代完成。
*/

pub fn resize_images_sequentially() {
    // 加载一个图像集合
    let images = vec![
        "image1.png",
        "image2.png",
        "image3.png",
        ...
    ];

    for image_path in images {
        // 从磁盘加载图像
        let img = image::open(image_path).expect("Failed to open the image");

        // 调整图像大小
        let resized_img = resize_image(img);

        // 将调整大小的图像保存到磁盘
        let output_path = format!("resized_{}", image_path);
        resized_img
            .save(output_path)
            .expect("Failed to save the resized image");
    }
}

/*
使用并行性，我们可以将调整大小的任务分配到多个cpu内核，从而允许我们同时处理多个图像。Rust的标准库包含了有用的多线程特性，所以我们可以以一种内存安全的方式轻松实现多线程：
*/

pub fn resize_images_in_parallel() {
    // 加载一个图像集合
    let images = vec![
        "image1.png",
        "image2.png",
        "image3.png",
        ...
    ];

    let mut handles = vec![];

    for image_path in images {
        // 为每个图像处理任务生成一个新线程
        handles.push(thread::spawn(move || {
            // 从磁盘加载图像
            let img = image::open(image_path).expect("Failed to open the image");

            // 调整图像大小
            let resized_img = resize_image(img);

            // 将调整大小的图像保存到磁盘
            let output_path = format!("resized_{}", image_path);
            resized_img
                .save(output_path)
                .expect("Failed to save the resized image");
        }));
    }

    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }
}
