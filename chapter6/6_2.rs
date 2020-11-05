fn modify(mut v:Vec<u32>)->Vec<u32>{
	v.push(45);
	v
}
fn main(){
	let v=vec![1,2,4];
	let v=modify(v);
	println!("{:?}",v);
}
