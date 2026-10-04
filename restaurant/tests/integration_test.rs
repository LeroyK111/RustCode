/*
创建一个tests专用 测试用例模块
*/

// 补充说明：这是独立的集成测试文件。
use restaurant::add_two;

/*
cargo test --test integration_test -- --no-capture
这里就是集成测试了

src和tests 是同级目录
*/

#[test]
fn it_adds_two() {
    // add_two 只接收一个参数，表示“这个数加二”
    let result = add_two(2);

    println!("调用 add_two，返回值：{result}");

    assert_eq!(result, 4);
}

/*
单元测试: 功能函数和测试模块 是一个文件
集成测试: 在src的同级目录下, 还有一个 tests目录 专门存放集成测试文件
*/
