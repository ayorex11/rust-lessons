/*fn length(s: &String) -> usize {     // & borrows, does not take ownership
    s.len()
}

fn add_exclaim(s: &mut String) {     // &mut borrows with permission to change
    s.push('!');
}

fn main() {
    let mut text = String::from("hey");
    let n = length(&text);           // pass a reference with &
    add_exclaim(&mut text);          // pass a mutable reference with &mut
    println!("{text} {n}");          // prints: hey! 3
}*/
fn count_vowels(s: &String) -> usize{
    let mut count = 0;
    for c in s.chars(){
        if c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'{
            count += 1;
        }
    }
    count
}

fn shout(s: &mut String){
    s.push_str("!!!");
}

fn describe(s: &String) -> String{
    let length = s.len();
    format!("'{s}' has {length} characters")
}

fn main(){
    let mut word = String::from("education");
    let vowels = count_vowels(& word);
    shout(&mut word); // modifies in place
    let description = describe(& word);
    println!("Vowels: {vowels}");
    println!("{description}");
    println!("{word}");

}

/*
let r1 = &mut word;
let r2 = &mut word;
println!("{r1} {r2}");
*/ // we can't write this in main because it is a rule in rust that you can't borrow two mutable copies
// at the same time within the same scope.