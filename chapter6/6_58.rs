fn main(){
	let v=vec![1,2,3];
	{
		let mut _iterator=v.into_iter();
		loop{
			match _iterator.next(){
				Some(i) =>{
					println!("{}",i);
				}
				_ => break,
			}
		}
	}
}
