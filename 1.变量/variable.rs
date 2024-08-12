// 变量相关的操作
// 不可变变量不能被修改数值
// 可变变量可以修改数值（mut）
// 变量的Shadowing可以重复利用变量
fn main() {
    
    // 申明不可变变量
    let x = 12;
    println!("{}", x);

    // 该语句编译失败；不可变变量不能修改其数值
    // x = 13;


    // 申请可变变量
    let mut y = 13;
    println!("{}", y);  // output=13
    y = 14;
    println!("{}", y); // output=14


    // 变量的隐藏
    let z = "123";
    let z = z.len();
    println!("{}", z); // output=3
}