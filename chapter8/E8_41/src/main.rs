fn main() {
    let mut vec=Vec::with_capacity(10);
    for i in 0..10{vec.push(i);}
    vec.truncate(0);
    assert_eq!(10,vec.capacity());
    for i in 0..10{vec.push(i);}
    vec.clear();
    assert_eq!(10,vec.capacity());
    vec.shrink_to_fit();
    assert_eq!(0,vec.capacity());
    for i in 0..10{
        vec.push(i);
        print!("{:?}",vec.capacity());
    }
}
