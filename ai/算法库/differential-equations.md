# differential-equations：高性能的微分方程库

为了利用 Rust 的性能和开发效率，作者将轨道力学数值模拟程序转换为 Rust 语言编写，但是发现需要更全面的微分方程求解器。现有的 Rust 中替代像 Scipy 的 solve_ivp 这样工具的方案，缺乏诸如事件处理、解输出控制，以及在设计上的更大灵活性等功能。

为了解决这个问题，作者开发了differential-equations，这是一个用 Rust 编写的用于数值求解常微分方程（ODE）、时滞微分方程（DDE）和随机微分方程（SDE）的库。

该库采用 trait 驱动架构，提供了符合 Rust 习惯且适应性强的 API，允许轻松互换不同的数值积分方法，并定制求解过程。已实现的功能包括：
用户通过 trait 定义自己的微分系统。
事件处理：具备在积分过程中定义并精确检测特定事件的能力。
解输出控制：对步间输出进行细粒度管理。
与 Polars 集成：可将解转换为 Polars 数据框。
以及其他更多功能Github 仓库：https://github.com/Ryan-D-Gast/differential-equations


