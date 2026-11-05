fn main() {
    const BOILING_POINT_C: f64 = 100.0;

    let temp = 32.0;

    let conv_to_f = celsius_to_fahrenheit(BOILING_POINT_C);

    println!("Boiling point: {conv_to_f} F");

    let conv_to_c = fahrenheit_to_celsius(temp);

    println!("{temp} F is now: {conv_to_c} C");

    let temp = conv_to_c;
    
    println!("temp is now {temp}");

}


fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f-32.0) /1.8

}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 1.8) + 32.0

}