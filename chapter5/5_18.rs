fn foo(s : String)->String{
	let w=" world".to_string();
	s+&w
}
fn main(){
	let s="Hello".to_string();
	let SS=foo(s);
	println!("{:?}",SS);
}
