fn main() {
    let sentence = String::from("ownership makes rust safe");
    println!("{}", last_word(&sentence));
    println!("{}", last_word("single"));
}

fn last_word(s: &str) -> &str {      // taking &str accepts String refs AND string literals
    let bytes = s.as_bytes();
    let mut last_index = 0;
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            last_index = i+1;
        }
    }
    &s[last_index .. ]
}

// This won't run due to mutable and immutable borrow
/*fn main() {
    let mut s = String::from("hello world");
    let word = last_word(&s); // immutable borrow
    s.clear(); // mutable usage
    println!("{word}");
}*/

// returning borrowed value when there's no value to borrow from
/*fn make_greeting() -> String { //remove '&'
    let s = String::from("hi");
    s
}

fn main() {
    let g = make_greeting();
    println!("{g}");
}*/