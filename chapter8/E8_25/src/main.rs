fn main() {
    let b="Hello\trust\t";
    assert_eq!(b.replace("\t"," "),"Hello rust ");
    assert_eq!(b.replace("\t"," ").trim(),"Hello rust");
    let s="this is new new 123";
    assert_eq!(s.replace("new","old"),"this is old old 123");
    assert_eq!(s.replacen(char::is_numeric,"new",1),"this is new new new23");
}
