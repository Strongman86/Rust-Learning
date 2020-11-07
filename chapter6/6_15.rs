type MathOp=fn (i32,i32)->i32;
fn math(op:MathOp,a:i32,b:i32)->i32{
	println!("{:p}",op);
	op(a,b)
}
fn sum(a:i32,b:i32)->i32{
        a+b
}
fn product(a:i32,b:i32)->i32{
        a*b
}
fn main(){
        let (a,b)=(3,4);
        println!("{:?}",math(sum,a,b));
        println!("{:?}",math(product,a,b));
}



