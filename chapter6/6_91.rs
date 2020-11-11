fn main(){
	let arr=[1,2,3,4,5];
	let sum=arr.iter().step(2).fold(0,|acc,x| acc+x);
	println!("{:?}",sum);
}
