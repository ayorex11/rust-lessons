/*fn main() {
    let word = String :: from("Hello");
    let byte = first_word(&word);
    println!("{byte}")
}

fn first_word(s : &String) -> usize {
    let byte = s.as_bytes(); // converting the string to bytes
    for (i, &item) in byte.iter().enumerate(){ // iter returns each value, enumerate wraps it in a tuple that gives us the index and corresponding value

        if item == b' '{ // we search for the value that has the space in byte literal
            return i; // when we find it we return the index
        }
    }
    s.len() // else we  return the length of the string instead

}*/ //Instead of this we can use string slicing 

/*fn main(){
    let s = String::from("hello world");
    let hello = &s[0..5]; // calls the reference to s and uses the start stop method
    let world = &s[6..11]; // start = first index, stop = one greater than last index
}*/ // with this knowledge we can know rewrite our initial function to return the first word before the space
// initially, we were trying to get the index of where the space was now  we can get the word

fn main() {
    let word = String :: from("Hello World!");
    let byte = first_word(&word);
    println!("{byte}")
}

fn first_word(s : &String) -> &str { // return &str instead of usize
    let byte = s.as_bytes(); // converting the string to bytes
    for (i, &item) in byte.iter().enumerate(){ // iter returns each value, enumerate wraps it in a tuple that gives us the index and corresponding value

        if item == b' '{ // we search for the value that has the space in byte literal
            return &s[..i]; // return the first word before the space using the starting index amd the index
            // of where space in byte literal was found
        }
    }
    &s[..] // return the whole word if there's no space

}

