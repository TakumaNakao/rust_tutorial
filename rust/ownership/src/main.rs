fn main() {
    let mut s = String::from("hello");

    change(&mut s);

    println!("{}", s);

    let s = String::from("hello world");

    let word = first_word(&s);

    println!("the first word is: {}", word);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
