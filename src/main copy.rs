enum Color {
    Red,
    Green,
    Blue,
    Yellow,
}

impl Color {
    fn is_green(&self) -> bool {
        if let Color::Green = self {
            return true;
        }
        return false;
    }

    fn is_green_parts(&self) -> bool {
        match self {
            Color::Red => false,
            Color::Green => false,
            Color::Blue => true,
            Color::Yellow => true,
        }
    }
}

fn print_colors(color: Color) {
    match color {
        Color::Red => println!("red"),
        Color::Green => println!("green"),
        Color::Blue => println!("blue"),
        Color::Yellow => println!("yellow"),
    }
}

fn main() {
    // let file = std::fs::read_to_string("lines").unwrap();

    // file.lines()
    //     .enumerate()
    //     .filter(|(i, _)| i % 2 == 0)
    //     .skip(2)
    //     .take(2)
    //     .for_each(|(_, line)| println!("{}", line));

    let foo = Color::Green;

    foo.is_green();
}
