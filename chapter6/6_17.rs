fn sum(a:i32,b:i32)->i32{
        a+b
}
fn product(a:i32,b:i32)->i32{
        a*b
}
type MathOp=fn(i32,i32)->i32;
fn math(op:&str,a:i32,b:i32)->MathOp{
	match op{
		"sum" => sum(a,b),//need to use function pointer
		_ => product(a,b)
	}
}
fn main(){
        let (a,b)=(3,4);
        println!("{:?}",math("sum",a,b));
        println!("{:?}",math("product",a,b));
}

