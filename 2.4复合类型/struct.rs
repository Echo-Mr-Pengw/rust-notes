// 结构体相关
#[derive(Debug)]
struct User {
	name: String,
	age: u32,
}
fn main() {
	// 初始化必须初始化结构体中的所所有字段
	// 初始化字段顺序与结构体字段顺序无关
	let u1 = User{
		name: String::from("小明"),
		age: 13,
	};
	println!("u={:#?}, name={}, age={}", u1, u1.name, u1.age);

	// 修改结构体中的字段值
	let mut u2 = User{
		name: String::from("小红"),
		age: 22,
	};
	u2.age = 23;
	println!("u={:#?}, name={}, age={}", u2, u2.name, u2.age);

	// 结构体的更新语法
	let u3 = User {
		..u2
	};
	println!("u={:#?}, name={}, age={}", u3, u3.name, u3.age);
}