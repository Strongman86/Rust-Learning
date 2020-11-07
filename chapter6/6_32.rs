#![feature(fn_traits)]
fn main(){
	let s="hello";
	let mut c=||{println!("{:?}",s)};
	c();
	c();
	c.call_mut(());
	c.call_once(());
	c();
	println!("{:?}",s);
}
