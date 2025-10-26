# RustedSciThe用于符号和数值计算。

RustedSciThe用于符号和数值计算。它可以解析字符串表达式为符号表达式/符号函数,计算符号导数或将符号表达式转换为常规Rust函数。它还可以计算符号Jacobian矩阵,并使用BDF和Backward Euler方法求解刚性常微分方程组的初值问题,使用Newton迭代法求解非刚性常微分方程和边值问题。

该库的主要特性包括:

解析字符串表达式为符号表达式/函数对符号表达式/函数
进行符号/解析微分比较解析导数和数值导数
计算偏导数向量
将符号表达式/函数(包括导数)转化为常规Rust函数
计算符号/解析Jacobian矩阵并转换为函数形式
使用解析Jacobian矩阵的Newton-Raphson方法、Backward Euler方法和BDF方法求解刚性常微分方程组
使用RK45和DP等经典方法求解非刚性常微分方程
使用Newton-Raphson方法求解常微分方程的边值问题

https://github.com/Gleb-Zaslavsky/RustedSciThe