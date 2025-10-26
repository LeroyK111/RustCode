// Maplit crate 用于生成容器类型的宏，对应于标准库中的容器类型。这在很大程度上是个人偏好的问题，因为容器已经有from和from_iter方法。
let a = btreemap! {
    "a" => vec![1, 2, 3],
    "b" => vec![4, 5, 6],
    "c" => vec![7, 8, 9],
};
// vs
let b = BTreeMap::from([
    ("a", vec![1, 2, 3]),
    ("b", vec![4, 5, 6]),
    ("c", vec![7, 8, 9]),
]);

