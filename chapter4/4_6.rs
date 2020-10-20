fn main(){
	let x:i32;
	let mut y=0;
	loop{
		y+=1;
		if y>3{
			x=2;
			break;
		}
	}
	println!("{}",x);
}
