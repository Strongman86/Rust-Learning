fn main() {
    assert_eq!("Hello\trust\t".trim_matches('\t'),"Hellorust");
    assert_eq!("Hello11rust11".trim_matches('1'),"Hellorust");
    assert_eq!("Hello11rust11".trim_matches(char::is_numeric),"Hellorust");
    let x:&[char]=&['1','2'];
    assert_eq!("12foo12".trim_left_matches(x),"foo");
}
