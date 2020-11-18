fn main(){
	let tao='道';
	let tao_u32=tao as u32;
	assert_eq!(36949,tao_u32);
	println!("U+{:x}",tao_u32);
	println!("{}",tao.escape_unicode());
	assert_eq!(char::from(65),'A');
	assert_eq!(std::char::from_u32(0x9053),Some('道'));
//	assert_eq!(std::char::from_u32(12901010101),None);

}
