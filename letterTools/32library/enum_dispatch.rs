/*
Rust通过静态和动态分派特性来支持多态性。如果你使用过动态分派，就知道它会对程序的性能产生负面影响，因为trait的实现是在运行时通过虚函数表查找的。而 enum_dispatch 库使用枚举将动态分派转换为静态分派。假设我们有这样一个trait和它的实现：
*/

pub trait ReturnsValue {
    fn return_value(&self) -> usize;
}

pub struct Zero;

impl ReturnsValue for Zero {
    fn return_value(&self) -> usize {
        0
    }
}

pub struct Any(usize);

impl ReturnsValue for Any {
    fn return_value(&self) -> usize {
        self.0
    }
}


// 在这个例子的测试中，我们使用了动态分派：

#[test]
fn derive_dispatch_dynamic() {
    let values: Vec<Box<dyn ReturnsValue>> = vec![Box::new(Zero {}), Box::new(Any(5))];

    assert_eq!(
        values
            .into_iter()
            .map(|dispatched| dispatched.return_value())
            .collect::<Vec<_>>(),
        vec![0, 5]
    );
}

// 现在让我们使用enum_dispatch：
#[enum_dispatch::enum_dispatch]
pub trait ReturnsValue {
    fn return_value(&self) -> usize;
}

// trait implementations are same

#[enum_dispatch::enum_dispatch(ReturnsValue)]
pub enum EnumDispatched {
    Zero,
    Any,
}

#[test]
fn derive_dispatch_static() {
    let values = vec![EnumDispatched::Zero(Zero {}), EnumDispatched::Any(Any(5))];

    assert_eq!(
        values
            .into_iter()
            .map(|dispatched| dispatched.return_value())
            .collect::<Vec<_>>(),
        vec![0, 5]
    );
}