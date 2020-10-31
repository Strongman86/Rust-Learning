#[derive(Debug)]
struct Foo<'a>{
	part:&'a str,
}
impl<'a>Foo<'a>{
	fn split_first(s:&'a str)->&'a str{
		s.split(',').next().expect("Cloud not find a ','")
	}
	fn new(s:&'a str)->Self{
		Foo{part:Foo::split_first(s)}
	}
}
fn main(){
	let words=String::from("Sometimes think,the greatest sorrow than older");
	println!("{:?}",Foo::new(words.as_str()));
}
