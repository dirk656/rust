struct Person{
    name: String,
    color:String,
    age:i32,
}
// &string &str
fn print (data: &str){
    println!("{}",data);
}
// &string
fn print2 (data:&String){
    println!("{}",data);
}
fn main ()
{
    let name: String = String::from("value c++");
    let course: String = "Rust".to_owned();
    let new_name = name.replace("c++","cpp");
    println!("{name}{course}{new_name}");

    // string &str 转换
    let name = "feng".to_string();
    let age = 18;
    let color = "yellow".to_string();
    let person = Person{
        name: name,
        color: color,
        age: age,
    };
    println!("{}{}{}",person.name,person.color,person.age);



    // func
    let value = "value".to_string();
    print(&value);
    print("value");
    print2(&value);


}