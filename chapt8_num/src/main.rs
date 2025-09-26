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
fn print_building(data:BuildingLocation){
    match data{
        BuildingLocation::Number(num) => println!("number:{}",num),
        BuildingLocation::Name(name) => println!("name:{}",name),
    }
}

fn main ()
{
    print_enum(Color::Red);
    print_enum(Color::Green);
    print_enum(Color::Blue);
    let num = 1;
    let name = "building A".to_string();
    print_building(BuildingLocation::Name(name));
    print_building(BuildingLocation::Number(num));
}