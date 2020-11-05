fn gcd(a:u32,b:u32)->u32{
	if(b==0){
		return a;
	}
	return gcd(b,a%b);
}
fn main(){
	let g=gcd(40,60);
	assert_eq!(20,g);
}
