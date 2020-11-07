fn hello(){
	println!("Hello,function pointer!");
}
fn main(){
	let fn_Str:fn()=hello;
	println!("{:p}",fn_Str);
	let other_str=hello;
	//println!("{:p}",other_str);
	hello();
	other_str;
	fn_Str();
	(fn_Str)();
}
