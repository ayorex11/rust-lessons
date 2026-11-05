
fn main() {
    let scores: [i32; 6] = [72, 85, 90, 64, 88, 79];
    let (high, low, average) = analyze(scores);
    println!("High -> {high}, low -> {low} and average -> {average}");
    let grade = grade(average);
    println!("grade {grade}");

    let grid = [[5, 3, 8], [2, 0, 7], [9, 4, 6]];
    'outer: for row in 0..3 {
        for col in 0..3 {
            if grid[row][col] == 0 {
                println!("Found zero at row {row}, col {col}");
                break 'outer;
            }
        }
    }
}

fn analyze(data: [i32; 6]) -> (i32, i32, f64) {
    let length_of_scores = data.len();
    //let final_range = length_of_scores + 1;
    let mut highest = data[0];
    let mut lowest = data[0];
    let mut total = 0;
    let average : f64;
    for number in 0..length_of_scores{
        if data[number] > highest {
            highest = data[number];
        }
        if data[number] < lowest{
            lowest = data[number];
        }
        total += data[number];

    }
    average = (total as f64) / (length_of_scores as f64);
    let tup : (i32, i32, f64) = (highest, lowest, average );
    return tup;
}

fn grade (avg:f64) -> &'static str{
    if avg >= 90.0{
        return "A";
    } else if avg >= 80.0{
        return "B";
    } else if avg >= 70.0{
        return "C";
    } else {
        return "D";
    }
}
