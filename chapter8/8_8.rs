fn main(){
	let str="boros";
	let mut chars=str.chars();
	assert_eq!(Some('b'),chars.next());
	assert_eq!(Some('o'),chars.next());
	assert_eq!(Some('r'),chars.next());
	assert_eq!(Some('o'),chars.next());
	assert_eq!(Some('s'),chars.next());
	let mut bytes=str.bytes();
	assert_eq!(Some(98),bytes.next());
	assert_eq!(Some(111),bytes.next());
	assert_eq!(Some(114),bytes.next());
	assert_eq!(Some(195),bytes.next());
	assert_eq!(Some(182),bytes.next());
	assert_eq!(Some(115),bytes.next());
}
