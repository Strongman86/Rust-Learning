fn main(){
	let a=Some("Hello".to_string());
	match a{
		Some(s)=>println!("{:?}",s),
		_=>println!("None"),
	}
}
