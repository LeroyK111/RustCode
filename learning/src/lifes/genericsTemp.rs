/*
编译器: 对于泛型, 处理是全部类型的硬编码, 所以不会拖累性能
*/

fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largestAdv<T: PartialOrd>(list: &[T]) -> &T {
    /*
        泛型: 但是需要合适的函数申明
    */
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Point<T, U> {
    x: T,
    y: U,
}

fn mixT() {
    struct Point<X1, Y1> {
        x: X1,
        y: Y1,
    }

    impl<X1, Y1> Point<X1, Y1> {
        fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
            Point {
                x: self.x,
                y: other.y,
            }
        }
    }

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

pub fn test() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);

    // println!("The largest number is {result}");

    // 多种泛型
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };

    // 枚举泛型
    enum Option<T> {
        Some(T),
        None,
    }

    // 方法定义中的泛型

    struct Point1<T> {
        x: T,
        y: T,
    }

    impl<T> Point1<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    impl Point1<f32> {
        fn distance_from_origin(&self) -> f32 {
            // powi 次方
            // sqrt 开方
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    let p = Point1 { x: 5.0, y: 10.0 };

    // println!("p.x = {}", p.x());
    // println!("distance_from_origin = {}", p.distance_from_origin());

    // 混合?
    mixT()
}
