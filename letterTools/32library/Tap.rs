
let val = last(
  third(
    second(first(original_value), another_arg)
  ),
  another_arg,
);


let val = original_value
  .pipe(first)
  .pipe(|v| second(v, another_arg))
  .pipe(third)
  .pipe(|v| last(v, another_arg));


/*
! Tap crate可以将函数调用链从前缀表示法转换为后缀表示法，这可以使你编写更具可读性的代码。
*/




