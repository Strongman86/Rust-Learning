#[derive(Debug,Copy,Clone)]
struct Book<'a>{
	name:&'a str,
	isbn: i32,
	version:i32,
}
fn main(){
	let book=Book{
		name:"Rust",
		isbn:20201114,
		version:1,
	};
	let book2=Book{
		isbn:2020,
		version:2,	
		..book
	};
	println!("{:?}",book);
	println!("{:?}",book2);
}
