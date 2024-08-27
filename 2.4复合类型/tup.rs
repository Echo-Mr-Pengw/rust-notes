// 元组相关的
fn main() {
	// 写法1
	let tup = (1.01, true, 3);
	// tup解构到xyz上
	let (x, y, z) = tup;
	println!("x={}, y={}, z={}", x, y, z);

	// 写法2
	let (q,w ,e) = (1, 2.2, 3);
	println!("q={}, w={}, e={}", q, w, e);

	let (a, b) = cal(11, 22);
	println!("a={}, b={}", a, b);
}

fn cal(x: i32, y: i32) -> (i32, i32) {
	(x, y)
}