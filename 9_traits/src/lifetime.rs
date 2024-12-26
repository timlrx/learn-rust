use std::fmt::Display;

pub fn run_lifetime() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");

    let val = get_x_or_zero(&10, &20);
    println!("The value is {}", val);

    let result2 = longest_with_an_announcement(string1.as_str(), string2, "Hello");
    println!("The longest string is {result2}");
}

static ZERO: i32 = 0;

pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// If y is not returned, there's no need to specify the lifetime of y
pub fn get_x_or_zero<'a>(x: &'a i32, y: &i32) -> &'a i32 {
    if *x > *y {
        x
    } else {
        &ZERO
    }
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct Foo<'a> {
    bar: &'a i32,
}
fn baz<'a>(f: &Foo<'a>) -> &'a i32 {
    f.bar
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest() {
        let string1 = String::from("abcd");
        let string2 = "xyz";

        let result = longest(string1.as_str(), string2);
        assert_eq!(result, "abcd");
        assert!(result == "abcd");
    }

    #[test]
    fn another() {
        let string1 = String::from("long string is long");
        {
            let string2 = String::from("xyz");
            let result = longest(string1.as_str(), string2.as_str());
            assert_eq!(result, "xyz", "The longest string is `{}`", result);
        }
    }
}
