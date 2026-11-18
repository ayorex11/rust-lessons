fn main (){
    let data = [4, 8, 15, 16, 23, 42];

    match find_index(&data, 15) {
        Some(i) => println!("found 15 at index {i}"),
        None => println!("15 not found"),
    }
    match find_index(&data, 99) {
        Some(i) => println!("found 99 at index {i}"),
        None => println!("99 not found"),
    }

    println!("{:?}", add_one(Some(5)));
    println!("{:?}", add_one(None));

    let result = safe_divide(20, 4);
    let double = match result {
        Some(n) => n * 2,
        None => 0,
    };
    println!("20 / 4 doubled: {double}");
    println!("20 / 0: {:?}", safe_divide(20, 0));
}

fn find_index(items: &[i32], target: i32) -> Option<usize>{
    for (i, itm) in items.iter().enumerate(){
        if *itm == target{
            return Some(i);
        }
    }
    None
}

fn add_one(x: Option<i32>) -> Option<i32>{
    match x {
        Some(x) => Some(x + 1),
        None => None
    }
}

fn safe_divide(a: i32, b: i32) -> Option<i32>{
    if b == 0{
        None
    } else {
        Some(a / b)
    }
}

// the book say Option<T> is safer than the null essentially because it ensures that there is a value
// and if there is not it safely returns None, if there is a value it returns it with Some<T>
// with Option<T> The compiler at run time does not know if T has a value or not.