/*
! 观察者模式
观察者模式介绍
观察者模式是一种软件设计模式，它允许对象(通常称为主题)维护一个称为观察者的依赖项列表，并自动通知它们任何状态更改。
许多语言都有这种模式，要么是内置的，要么是在标准库中。

1，首先是Observable，它可以是一个接口，被观察的对象。被观察对象拥有一个观察者列表。

2，Observer，观察者。这也可以是一个接口。

3，ConcreteObservable，保存状态的具体类。如果状态中有任何变化，则调用setState方法，然后调用notify方法反过来通知观察者。

4，ConcreteObserverA/B，处理状态变化的具体观察者。
*/

trait Observer {
    fn update(&self, data: &str);
}

struct Subject<'a> {
    observers: Vec<&'a dyn Observer>,
    state: String,
}

impl<'a> Subject<'a> {
    /*
    1，在Subject结构体中，观察者必须具有与整个结构体相同的生命周期。

    2，new方法，即构造函数，非常简单。

    3，attach方法，将观察者加入列表中。

    4，detach方法，使用指定的闭包进行筛选。闭包返回true的每个元素都保留在vector中，当它返回false时，这些元素被删除。

    5，notify方法遍历每个观察者，并在每个观察者上调用update方法。

    6，set_state方法改变状态，并调用notify，以便每个观察者都能对状态变化做出反应。
        */
    fn new(state: String) -> Self {
        Self {
            observers: Vec::new(),
            state: state,
        }
    }

    fn attach(&mut self, observer: &'a dyn Observer) {
        self.observers.push(observer);
    }

    fn detach(&mut self, observer: &dyn Observer) {
        self.observers.retain(|o| !std::ptr::eq(*o, observer));
    }

    fn notify(&self) {
        for o in &self.observers {
            o.update(&self.state);
        }
    }

    fn set_state(&mut self, state: String) {
        self.state = state;
        self.notify();
    }
}

struct ConcreteObserver {
    name: String,
}

impl Observer for ConcreteObserver {
    fn update(&self, data: &str) {
        println!("{} received data: {}", self.name, data);
    }
}

fn main() {
    /*
    1，我们用一些初始数据实例化Subject。

    2，然后定义两个观察者，它们都是concreteobserver。每一个都有一个不同的名称。

    3，然后我们需要将它们附加到Subject。

    4，set_state方法被调用，这会将状态变化发送给两个观察者，并将其打印出来。

    5，为了确保detach方法正确运行，我们分离第二个观察者。

    6，我们再调用一次set_state方法，我们现在应该只得到一条打印语句。
        */
    let mut subject = Subject::new("initial data".to_string());

    let observer1 = ConcreteObserver {
        name: "Observer 1".to_string(),
    };

    let observer2 = ConcreteObserver {
        name: "Observer 2".to_string(),
    };

    subject.attach(&observer1);
    subject.attach(&observer2);

    subject.set_state("updated_data".to_string());

    subject.detach(&observer2);

    subject.set_state("Again updated data".to_string());

    subject.detach(&observer1);
}
