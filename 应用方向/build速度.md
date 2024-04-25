# 为什么Rust的构建时间很慢？

Rust是一种高性能且对开发人员友好的编程语言，Rust因其安全性、速度、并发性和可编程性而在编程界获得了关注和赞誉。它也被证明是构建企业应用程序的绝佳选择。与C/C++等语言相比，Rust对于项目来说更容易上手，因为C/C++不那么内存安全，开发人员需要多年的经验才能开始工作，因此Rust被证明是一个更好的选择。

近年来，它也是开发社区中最受欢迎的编程语言，因为它为编程语言做出了创新和高效的设计，例如零成本抽象和所有权，关注性能而不以可编程性为代价。

由于语言设计的本质，Rust构建/编译时间相当慢，可能会阻碍开发人员的生产力：引入缓慢的反馈循环，作为编译时间的直接产物。下面的漫画总结了编译时间的问题，并且与这个场景非常相关。开发人员等待代码编译的时间越长，他们在产品上的工作就越少。这将影响整个发布时间，并可能在整个产品发布过程中造成蝴蝶效应。

为了帮助缓解这个问题，这篇文章将介绍各种优化Rust构建时间的策略，然后将这些策略与GitHub Actions合并。这将使Rust开发人员能够更快地进行迭代，从而帮助他们高效地完成项目。

## C++ 中的问题在 Rust 中仍然存在吗
这是 Reddit 上的一个讨论帖，主要讨论集中在 C++ 中存在的一些问题是否仍然存在于 Rust 中，以及这些问题如何影响开发者使用 Rust。具体问题包括：

泛型和模板的单态化：C++ 的模板和 Rust 的泛型都需要通过单态化处理，这会导致编译时间长和生成的二进制文件大。避免这一问题需要以完全不同的方式重写代码。
对 libc 的依赖：两种语言的标准库都依赖于平台的 libc，这不仅导致了典型的二进制文件体积大，还带来了各种开发难题。
RAII 机制和资源处理错误：C++ 和 Rust 都使用 RAII 机制管理资源，但在资源释放时不能很好地处理错误，尤其是在文件关闭可能报错的情况下。
编译和测试时间相似：尽管Rust在许多方面提供了改进，但在编译和测试时间上与 C++ 相似，仍然较长。
看来在大家眼里，从 C++ 到 Rust 虽有许多改进，但某些核心问题仍未得到解决


# Rust中缓慢构建速度的蝴蝶效应

缓慢的构建会在几个方面显著影响开发速度和生产力。在时间至关重要的企业项目中，这可能会对团队的工程习惯、发布节奏和未来的产品规划产生重大影响。如果应用程序构建太慢且不允许开发人员快速迭代，则会有以下影响：

- 更长的反馈循环

缓慢的构建导致更长的反馈循环。开发人员必须等待构建过程完成，然后才能测试更改或接收自动测试结果。如果使用的是Uffizzi，那么在这种情况下，构建和部署拉取请求的预览环境也可能需要更长的时间。

缓慢的构建时间(平均20分钟)导致上下文切换到其他任务也需要等待。这破坏了开发流程。当在构建和测试之后回到开发过程时，开发人员必须花一点额外的时间重新构建代码库，然后在程序上进行重复这个过程。

- 阻碍合作

由于较长的反馈循环，工程团队成员之间可能会出现脱节。这是由于单个开发人员自己花费了更长的时间进行开发，导致整个团队的速度比他们应该的要慢。在sprint中，同行之间共享的知识并不多，这减少了协作，减缓了产品的增长。

- 减少部署频率

由于没有及时发布足够的bug修复和特性，因此降低了总体部署频率。如果客户一直在等待某些错误修复或特定功能，这将直接影响客户满意度。业务敏捷性也会受到影响，因为业务本身不能足够快地达到设定的目标，并向客户提供理想的产品。反过来，作为研发和发布周期缓慢的直接产物，影响了企业对市场变化的反应能力。缓慢的发布周期意味着新功能得不到足够快的反馈。

- 降低代码质量

当构建花费很长时间时，开发人员在发布当天就会被时间所束缚，并且可能不太倾向于编写好的代码，从而导致低质量的代码合并。这可能导致总体上较低的代码质量，并增加引入错误或回归的可能性。

- 增加CI/CD成本

产品构建时间直接影响CI/CD成本。通过在构建过程中减少几分钟的时间，可以节省很多钱。这对于拥有大型项目或多个应用程序的企业来说尤其成问题，因为CI/CD成本影响会以数量级增加。


# 优化Rust应用程序构建的策略

以下优化Rust构建的策略各有利弊，由用户决定在他们的构建用例中什么最适合。用户需要考虑他们正在优化的构建是开发、发布、测试还是其他构建。找出每种构建优化的正确组合有助于用户顺利开发和发布。

Rust应用程序的发布构建往往比开发人员构建要慢得多。这是由于编译器在发布构建期间进行了优化，使应用程序的二进制文件尽可能最小。最后，用户必须决定自己的构建优化选择。以下是优化Rust应用程序构建的策略。这些策略可以相互配合使用：

## 有效的缓存利用率

缓存是最直接的，也是加快构建时间最关键的。通过缓存target目录和cargo registry，可以显著减少编译依赖项所花费的时间。

缓存target目录：该目录包含构建的构件，缓存它将节省后续构建的时间。


缓存cargo registry：这确保依赖项不会被不必要地重新下载或重新编译。

对于上述缓存配置，可以使用流行的https://github.com/Swatinem/rust-cache来简化为Rust应用程序构建设置和使用缓存的过程。

- name: Cache dependencies
  uses: Swatinem/rust-cache@v2.2.1
在介绍了上面的基本依赖项缓存之后，可以使用更智能的缓存sccache作为编译器缓存工具。它充当编译器包装器，并尽可能避免编译。在这种情况下，确保我们不仅缓存了依赖项，而且还缓存不需要在每次构建时重新编译的编译时构件。

- name: Configure sccache
  run: | 
      echo "RUSTC_WRAPPER=sccache" >> $GITHUB_ENV
        echo “SCCACHE_GHA_ENABLED=true" >> $GITHUB_ENV

- name: Run sccache-cache
  uses: mozilla-actions/sccache-action@v0.0.2
上面的一组github action设置了sccache环境变量，其中RUSTC_WRAPPER指示要使用哪个编译器包装器，SCCACHE_GHA_ENABLED设置sccache以使用github action缓存。

要了解有关sccache的更多信息，请查看https://github.com/mozilla/sccache/


## 并行编译

Rust支持开箱即用的并行编译，这允许利用多核处理器的强大功能，以成倍地加快构建过程。要启用并行编译，请在config.toml中设置codegen-units选项。

[profile.dev]
codegen-units = 1
代码单元或代码生成单元是将代码分成几个部分，以便并行地对每个部分执行编译，这将大大提高编译速度。这样做的缺点是，如果代码没有被分解和逐块编译，代码将无法得到优化。

增加代码单元的数量可能会导致错过一些潜在的优化，但可以通过将该值设置为1来优化运行时性能。这意味着代码库将被视为单个代码片段，并且不会进行并行编译。

[profile.release]
codegen-units = 1


## 覆盖配置文件

Rust中的构建系统预定义了一组配置选项，这些集合称为profiles。默认情况下，Rust为不同的目的使用不同的构建配置文件。

例如，在开发过程中构建项目时将使用dev配置文件。此配置文件优先考虑更快的构建时间，并启用损害性能的调试语句。要使用dev配置文件进行构建，请在命令行中运行cargo build。该命令不需要任何标志来指定这是一个开发构建，因为这是默认的构建选项。

release的目的是在向外界发布应用程序的最终版本时使用。因此，自然地，这个release以较慢的编译时间为代价，优先考虑生成二进制文件的速度。要使用发布配置文件进行构建，只需在项目的根目录中使用cargo build --release。

可以根据用户的需要，通过向config.toml添加配置来覆盖这些默认配置文件。例如，要降低release的优化级别，可以配置以下内容：

[profile.release]
opt-level = 2
codegen-units = 16
上面的配置将opt-level(优化级别)从3(默认值)减少到2。

opt-level是一个编译器设置，它控制优化过程中应用的级别，其中级别用数字表示。以下是设置及其含义：

opt-level = 0：没有优化。此设置优先考虑快速编译时间，使其适合于开发和调试，并牺牲运行时性能。


opt-level = 1：基本优化。在编译速度和运行时性能之间提供平衡，这对开发期间的增量构建很有好处。


opt-level = 2：更高级别的优化。以较慢的编译时间为代价，提高生成的二进制文件的运行时性能，用于版本构建，优化程度略低于最佳水平。


opt-level = 3：最高级别的优化。着重于最大化所生成二进制文件的性能。结果导致编译时间明显变慢，并且由于激进的优化而使调试变得困难。


除了设置opt-level之外，代码单元设置增加到16，允许在编译期间进行更多的并行化。


## 在Github Actions中应用配置，允许更快的发布构建以及有效的缓存利用率

考虑这样一个项目，它需要构建短暂预览优化版本。这个构建必须比通常的Rust release构建更快地完成，并且不需要完全优化，以便更快地创建Rust应用程序二进制文件，然后可以在短暂的环境中用于测试。

### Rust构建配置

考虑到上述情况，优化级别可以降低，不必是最高的，所以我们可以将opt-level设置为2而不是3(默认值)。考虑到我们希望构建仍然快一点，让我们通过将codegen-units设置为4来应用一些并行编译。对于临时环境构建来说，这是一个很好的配置，但创建自定义配置文件更有意义。

要创建自定义配置文件，让我们向Cargo.toml添加以下内容，它将创建一个名为ephemeral-build的新构建配置文件，其中包含我们需要的配置。

[profile.ephemeral-build]
opt-level = 2
codegen-units = 8
要使用临时构建配置文件，必须将其设置为在进行发布构建时使用的默认配置文件。这可以通过为Rust设置--cfg标志来完成，通过一个环境变量RUSTFLAGS导出该标志及其相关值，该变量将在运行时读取。

RUSTFLAGS=”--cfg profile=ephemeral-build” cargo build --release


### Dockerfile配置

以最可移植的方式发布应用程序的最佳方式是通过容器映像。下面的Dockerfile只接受构建应用程序的二进制文件，考虑到缓存优化是在Github Actions中完成的，image构建器不必再担心构建和缓存了。所有需要做的就是将二进制文件复制到image中，然后image就可以使用了，非常简单。

FROM alpine:latest

RUN apk update --quiet \
&& apk add -q --no-cache libgcc tini curl

COPY target/x86_64-unknown-linux-musl/release/app /bin/app
RUN ln -s /bin/app /app

ENTRYPOINT ["app"]
在上面的配置中，在必要的包更新之后，只有二进制文件被复制，然后进行符号链接以获得更好的访问。



### Github Actions配置

可以一起使用上述所有配置来创建应用程序映像的构建管道。构建管道通过使用前面提到的策略进行了优化，并且还生成了包含应用程序二进制文件的容器映像。下面是Github Action管道的样子：

name: Rust application ephemeral environment build

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  build:
    runs-on: ubuntu-latest

    steps:
    - name: Checkout repository
      uses: actions/checkout@v2

    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
          toolchain: stable
          override: true
          target: x86_64-unknown-linux-musl

    - name: Configure sccache env var and set build profile to ephemeral build
      run: | 
          echo "RUSTC_WRAPPER=sccache" >> $GITHUB_ENV
      echo “SCCACHE_GHA_ENABLED=true" >> $GITHUB_ENV
          echo “RUSTFLAGS=’--cfg profile=ephemeral-build’” >> $GITHUB_ENV

    - name: Run sccache-cache
      uses: mozilla-actions/sccache-action@v0.0.2

    - name: Run build
        uses: actions-rs/cargo@v1
        with:
            command: build
            args: --target x86_64-unknown-linux-musl --release
在上面启动管道时，发生的第一件事是Checkout存储库。


下一步将安装Rust。在这里，x86_64-unknown-linux-musl目标用于安装和构建，对于我们最终的容器映像构建，我们使用的基本映像是alpine:latest，为了让我们的应用程序在alpine容器中运行，我们需要将其构建到MUSL目标。


设置必要的环境变量


使用sccache用作Rust编译器包装器，RUSTC_WRAPPER使用Github Actions缓存。


RUSTFLAGS用于设置在进行发布构建时使用ephemeral-build进行构建。


上面的Github Actions配置优化了Rust应用程序的构建，专门用于临时环境。最终的应用程序构建经过了足够的优化，可以很容易地进行测试，并且构建得足够快，从而不会在迭代之间花费太多时间。这对于短暂的环境构建来说是完美的。
