fn main(){
	let s="Per Martin-lof";
	let (first,last)=s.split_at(12);
	assert_eq!("of",last);
	assert_eq!("Per Martin-l",first);
	let (first,last)=s.split_at(13);
}
