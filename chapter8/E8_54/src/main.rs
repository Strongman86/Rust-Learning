use std::collections::HashMap;
fn main() {
    let mut map:HashMap<&str,u32>=HashMap::new();
    map.entry("current").or_insert(2017);
    assert_eq!(map["current"],2017);
    *map.entry("current").or_insert(2017)+=10;
    assert_eq!(map["current"],2027);
    let last=2016;
    map.entry("next").or_insert_with(|| last+4);
    assert_eq!(map["next"],2020);
    assert_eq!(map.entry("current").key(),&"current");
}
