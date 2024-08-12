// 数值类型
// 整型、浮点型、序列、数值运算

fn main() {
    // 整数
    // i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize
    // 整型隐式申明
    // 默认是i32
    let x = 12; // 编译器类型推到为 i32
    println!("{}", x);

    // 显示申明
    let y: u16 = 13;
    println!("{}", y);


    // 浮点数
    // f32 f64(默认)
    // 整型隐式申明
    // 默认是f64
    let z = 1.2;
    println!("{}", z);

    let w: f32 = 1.23;
    println!("{}", w);
}