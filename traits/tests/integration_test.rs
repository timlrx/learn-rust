use traits::lifetime::longest;

#[test]
fn test_my_test() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    assert_eq!(result, "abcd");
}
