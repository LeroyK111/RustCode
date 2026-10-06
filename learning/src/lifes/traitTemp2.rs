// 定义共同能力
trait Speak {
    // 这里其实是 speak 的申明
    fn speak(&self) -> String;

    // 默认方法：所有实现 Speak 的类型都可以共用
    fn introduce(&self) -> String {
        format!("你好！我的叫声是：{}", self.speak())
    }
}

struct Dog {
    name: String,
}

struct Cat {
    name: String,
}

// 狗实现共同能力
impl Speak for Dog {
    fn speak(&self) -> String {
        format!("{}：汪汪！", self.name)
    }
}

// 猫实现共同能力
impl Speak for Cat {
    fn speak(&self) -> String {
        format!("{}：喵喵！", self.name)
    }
}

// dyn: dynamic 就是动态分发
fn createAnimal(isDog: bool) -> Box<dyn Speak> {
    if isDog {
        // 把狗移动到堆上的分配中，由 Box 拥有它。最初得到的是 Box<Dog>。
        Box::new(
            // 这里创建狗的实例
            Dog {
                name: String::from("小黑"),
            },
        )
    } else {
        Box::new(Cat {
            name: String::from("小白"),
        })
    }
}

/*

一旦你使用 Box 则 返回类型 只能是 Box<dyn Speak>

fn createAnimal(isDog: bool) -> Box<dyn Speak> {
    if isDog {
        let dog = Dog {
            name: String::from("小黑"),
        };

        let boxedDog: Box<Dog> = Box::new(dog);
        let animal: Box<dyn Speak> = boxedDog;

        animal
    } else {
        let cat = Cat {
            name: String::from("小白"),
        };

        let boxedCat: Box<Cat> = Box::new(cat);
        let animal: Box<dyn Speak> = boxedCat;

        animal
    }
}
*/

//
fn listen(animal: &dyn Speak) {
    println!("{}", animal.introduce());
}

pub fn test() {
    println!("=== 分别创建并调用 ===");

    let dog = createAnimal(true);
    let cat = createAnimal(false);

    /*
        dog 是一个盒子，但你要调用的是盒子里面对象的方法。

    Box 支持解引用，Rust 的方法调用会自动沿着它找到内部对象，并按方法需要借用它。因此可以直接写 dog.speak()，不用手动取出狗
        */
    println!("{}", dog.speak());
    println!("{}", cat.speak());

    println!("=== 通过共同接口调用 ===");

    /*
    as_ref 则是获取盒子内部的共享引用

    以下都是变种
    */
    //
    // let animalRef: &dyn Speak = dog.as_ref();
    // listen(animalRef);

    // !都是从右边 往 左边读
    // listen(&*dog);

    listen(dog.as_ref());
    listen(cat.as_ref());

    println!("=== 放进同一个集合 ===");
    /*
        Vec< ... >           可增长的集合
        Box< ... >           集合里的每个元素是一个拥有对象的盒子
        dyn Speak            对象通过 Speak 能力使用

        Vec<Box<dyn Speak>>
    */
    let animals = vec![dog, cat];

    for animal in &animals {
        println!("{}", animal.introduce());
    }
}


/*
以下是动态分发和静态分发.

静态分发 也是一种零成本抽象
*/

trait Speak1 {
    fn speak(&self);
}

struct Dog1;
struct Cat1;

impl Speak1 for Dog1 {
    fn speak(&self) {
        println!("汪汪");
    }
}

impl Speak1 for Cat1 {
    fn speak(&self) {
        println!("喵喵");
    }
}

// 静态分发
fn greet_static<T: Speak1>(animal: &T) {
    animal.speak();
}

// 动态分发
fn greet_dynamic(animal: &dyn Speak1) {
    animal.speak();
}

fn tes1() {
    let dog = Dog1;
    let cat = Cat1;

    greet_static(&dog);
    greet_static(&cat);

    greet_dynamic(&dog);
    greet_dynamic(&cat);
}