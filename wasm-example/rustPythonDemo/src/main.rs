/*
todo 测试使用 rustpython 构建

1.cargo add rustpython
2.cargo install rustpython
*/

use rustpython::vm::{compiler, VirtualMachine};
use rustpython::InterpreterConfig;

fn py_main() {
    let interp = InterpreterConfig::new().init_stdlib().interpreter();

    interp.enter(|vm| {
        let scope = vm.new_scope_with_builtins();
        let code = "print('Hello World!')";
        let code_obj = vm
            .compile(code, compiler::Mode::Exec, "<embedded>".to_owned())
            .unwrap();
        vm.run_code_obj(code_obj, scope).unwrap();
        // ✅ 直接结束，不写 Ok(())
    });
}

fn main() {
    py_main();
}
