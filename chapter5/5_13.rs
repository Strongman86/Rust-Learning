fn main(){
	let out_val=1;
	let out_sp="Hello".to_string();
	{
		let inner_val=2;
		out_val;
		out_sp;//quote semantic lose ownership
	}
	println!("{:?}",out_val);//domain
}
