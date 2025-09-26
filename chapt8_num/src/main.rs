enum Color{
    Red,
    Green,
    Blue,
}

enum BuildingLocation{
    Number(i32),
    Name(String),
}

fn print_enum(data:Color){
    match data{
        Color::Red => println!("red"),
        Color::Green => println!("green"),
        Color::Blue => println!("blue"),
    }
}


fn main ()
{
    print_enum(Color::Red);
    print_enum(Color::Green);
    print_enum(Color::Blue);
}