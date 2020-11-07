#[derive(Debug)]
struct User{
	name:&'static str,
	avatar_url:&'static str,
}
impl User{
	fn show(&self){
		println!("name:{:?}",self.name);
		println!("avatar:{:?}",self.avatar_url);
	}
}
fn main(){
	let user=User{
		name:"Alex",
		avatar_url:"https://avatar.com/alex"
	};
	User::show(&user);
	user.show();
}
