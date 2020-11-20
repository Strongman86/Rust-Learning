fn main(){
    	let mut a=String::from("foo");
    	println!("{:p}",a.as_ptr());
   	println!("{:p}",&a);
    	println!("-----------");
    	assert_eq!(a.len(),3);
    	a.reserve(10);
	assert_eq!(a.capacity(),13);
}

