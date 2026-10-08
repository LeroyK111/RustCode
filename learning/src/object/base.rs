mod collection {
    pub struct AveragedCollection {
        list: Vec<i32>,
        average: f64,
    }

    impl AveragedCollection {
        pub fn new() -> Self {
            Self {
                // 相当于初始胡函数的 值
                list: Vec::new(),
                average: 0.0,
            }
        }

        pub fn add(&mut self, value: i32) {
            self.list.push(value);
            self.update_average();
        }

        pub fn remove(&mut self) -> Option<i32> {
            let result = self.list.pop();

            if result.is_some() {
                self.update_average();
            }

            result
        }

        pub fn average(&self) -> f64 {
            self.average
        }

        fn update_average(&mut self) {
            if self.list.is_empty() {
                self.average = 0.0;
                return;
            }

            let total: f64 = self.list.iter().map(|&value| value as f64).sum();

            self.average = total / self.list.len() as f64;
        }
    }
}

use collection::AveragedCollection;

pub fn test() {
    let mut numbers = AveragedCollection::new();

    numbers.add(10);
    numbers.add(20);

    println!("添加后的平均值：{}", numbers.average());

    println!("移除的值：{:?}", numbers.remove());

    println!("移除后的平均值：{}", numbers.average());

    // 取消注释会报错：字段在 collection 模块中是私有的
    // numbers.list.push(100);
    // numbers.average = 999.0;
}
