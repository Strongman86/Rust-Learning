#![feature(fnbox)]
use std::boxed::FnBox;//The FnBox may be discarded by the new version rust
fn square()->Box<FnBox(i32)->i32>{
	Box::new(|i| i*i)
}
fn main(){
	let square=square();
	assert_eq!(4,square(2));
}
