#![feature(fn_traits)]
fn main(){
	let mut s="rush".to_string();
	{
		let mut c=|| s+"rust";
		c.call_once(());

	}
	println!("{:?}",s);
}
