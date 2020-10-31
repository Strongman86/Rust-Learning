fn main(){
	let a=Some("Hello".to_string());
	if let Some(s)=a{
		println!("{:?}",s);
	}
}
