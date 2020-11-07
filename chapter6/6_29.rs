fn main(){
	let env_var=1;
	let c:Box<Fn()->i32>=Box::new(|| env_var+2);
}
