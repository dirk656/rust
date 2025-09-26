enum Flavor{
    Spicy,
    Sweet,
    Sour,
}


struct Drink {
    flavor:Flavor,
    price:f64,
}
fn print_drink(drink:Drink){
    match drink.flavor {
        Flavor::Spicy => println!("spicy"),
        Flavor::Sweet => println!("sweet"),
        Flavor::Sour => println!("sour"),
    }
    println!("{}",drink.price);
    
}
// method
impl Drink {
    fn buy (&self){
        if self.price < 5.0 {
            println!("buying");
        } else {
            println!("too expensive");
        }
    }
}

fn main (){
    let drink = Drink{
        flavor:Flavor::Spicy,
        price:3.5,
    };
    println!("{}",drink.price);
    // ownership moved
    print_drink(drink);
    let drink = Drink{
        flavor:Flavor::Spicy,
        price:3.5,
    };
    drink.buy();
}
