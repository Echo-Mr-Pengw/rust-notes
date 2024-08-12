// 常量相关操作
// 常量被定义就不能修改
// 常量没有mut修饰，必须申明类型
// 常量命名：大写字幕，中间可用下划线分割

fn main() {
    // 常量需要显示申明类型
   // const MAX_VALUE = 12; // 编译报错：需要显示申明类型

    // 常量被定义后不能被修改
    const MIN_VALUE: u8 = 1;
    println!("{}", MIN_VALUE); // output=1

    // const MIN_VALUE: u8 = 2; // 编译报错
}