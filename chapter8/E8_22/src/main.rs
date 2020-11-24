fn main() {
    let v="absXXXabsYYYabs".matches("abs").collect::<Vec<&str>>();
    assert_eq!(v,["abs","abs","abs"]);
    let v="1abs2abs3".rmatches(char::is_numeric).collect::<Vec<&str>>();
    assert_eq!(v,["3","2","1"]);
    let v="abcXXXabcYYYabc".match_indices("abc").collect::<Vec<_>>();
    assert_eq!(v,[(0,"abc"),(6,"abc"),(12,"abc")]);
    let v="abcXXXabcYYYabc".rmatch_indices("abc").collect::<Vec<_>>();
    assert_eq!(v,[(12,"abc"),(6,"abc"),(0,"abc")]);
}
