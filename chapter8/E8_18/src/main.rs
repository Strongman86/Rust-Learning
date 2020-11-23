fn main() {
    let b="bananas";
    assert!(b.contains('a'));
    assert!(b.contains("an"));
    assert!(b.contains(char::is_lowercase));
    assert!(b.starts_with('b'));
    assert!(!b.ends_with("ana"));
}
