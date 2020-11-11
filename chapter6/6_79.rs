fn main(){
	let numbers=vec![1,2,3,4,5,6];
	let mut iter=numbers.into_iter();
	assert_eq!(Some(1),iter.next());
	assert_eq!(Some(2),iter.next());
	assert_eq!(Some(6),iter.next_back());
}
