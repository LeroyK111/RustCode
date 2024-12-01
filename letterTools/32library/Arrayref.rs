// 用于方便地从切片创建数组的宏。
let addr: &[u8; 16] = ...;
let mut segments = [0u16; 8];
// 基于数组
for i in 0 .. 8 {
    let mut two_bytes = [addr[2*i], addr[2*i+1]];
    segments[i] = read_u16_array(&two_bytes);
}
// 基于arrayref
for i in 0 .. 8 {
    segments[i] = read_u16_array(array_ref![addr, 2*i, 2]);
}
