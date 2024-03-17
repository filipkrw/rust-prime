fn excercise() {
    let path = std::env::args().nth(1).expect("Path not privded");
    let file = std::fs::read_to_string(path).expect("Error reading file");

    let lines = file.lines();

    lines
        .map(|line| match line.parse::<usize>() {
            Ok(line) => return line.to_string(),
            Err(_) => return String::from("Line not a number"),
        })
        .for_each(|line| println!("{}", line));
}

fn main() {
    excercise();
}
