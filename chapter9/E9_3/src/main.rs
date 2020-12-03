fn main() {
    let x=false;
    assert!(x,"false");
    let a=3;
    let b=28;
    debug_assert!(a+b==30,"a={},b={}",a,b);
}
