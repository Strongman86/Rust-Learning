fn main(){
	let env_var=1;
	let c=|| env_var+2;
	assert_eq!(c(),3);
}
