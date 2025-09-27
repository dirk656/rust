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
        if self.price < 10.0 {
            println!("buying");
        } else {
            println!("too expensive");
        }
    }
    fn price (price:f64) -> Self{
        Drink { flavor:Flavor::Sour, price }
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
     let sour = Drink{
        flavor:Flavor::Sour,
        price:20.0,
    };
    drink.buy();
    sour.buy();
}
