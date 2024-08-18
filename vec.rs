// vec的使用
fn main() {
    // 第一种申明
    // let v1: Vec<i32> = Vec::new(); // 如果后面不放入数，必须等申明类型
    // println!("{:?}", v1)


    // let v1 = Vec::new();
    // println!("{:?}", v1); // 报错

    // let mut v1 = Vec::new();
    // v1.push(12);
    // println!("{:?}", v1);  // 正常输出

    // 第二种
    // let v2 = vec![1, 2, 3];
    // println!("{:?}", v2);

    // 第三种
    // let v3 = vec![2;10];
    // println!("{:?}", v3);

    // 第四种，指定容量
    // let v4: Vec<i32> = Vec::with_capacity(10);
    // println!("{:?}", v4);


    // 访问
    let v1 = vec![1, 2, 3, 4];
    println!("{}", v1[0]);  // 通过索引访问，如果超出索引范围，则panic
    println!("{:?}", v1.get(0)); // 通过get访问，超时索引范围，不panic

    // 遍历
    for i in v1 {
        println!("{}", i);
    }
}