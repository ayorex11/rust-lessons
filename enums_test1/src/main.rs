enum Command{
    Quit,
    Move{ x: i32, y: i32 },
    Say(String),
    Paint(u8, u8, u8)

}

fn run(cmd: &Command) -> String{
    match cmd{
        Command::Quit => format!("Goodbye"),
        Command::Move { x, y } => format!("Moving to ({x}, {y})"),
        Command::Say(string) => format!("Saying: {string}"),
        Command::Paint(r,g, b ) => format!("Painting rgb ({r}, {g}, {b})")
        // if the Paint arm is deleted we have a non exhaustive case. We must handle all possible cases
        // of Command
    }
}

impl Command {
    fn is_quit(&self) -> bool { // method to check if variant is Quit.
        match self{
            Command::Quit => true,
            _ => false
        }
    }
}

fn main() {
    let commands = vec![ // creating a list where every type is of Command.
        Command::Quit,
        Command::Move { x: 10, y: -5 },
        Command::Say(String::from("hello")),
        Command::Paint(255, 0, 128)
    ];

    for cmd in &commands{
        println!("{}", run(cmd));
    }
    println!("first is_quit: {}", commands[0].is_quit());
    println!("last is_quit: {}", commands.last().unwrap().is_quit());
}
