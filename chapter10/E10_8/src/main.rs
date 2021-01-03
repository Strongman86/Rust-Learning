use regex::Regex;
fn main() {
    let re =Regex::new(r"(?x)
        (?P<year>\d{4}) #year
        -
        (?P<month>\d{2}) #month
        -
        (?P<day>\d{2})#day
    ").unwrap();
    let caps=re.captures("2021-01-01").unwrap();
    assert_eq!("2021",&caps["year"]);
    assert_eq!("01",&caps["month"]);
    assert_eq!("01",&caps["day"]);
    let after=re.replace_all("2021-01-03","$month/$day/$year");
    assert_eq!(after,"01/03/2021");
    println!("Hello, world!");
}
