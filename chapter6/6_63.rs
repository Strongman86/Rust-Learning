fn main(){
	let mut s="Hello".to_string();
	s.extend(&[' ','r','u','s','t']);
	println!("{:?}",s);
}
