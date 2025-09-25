
fn main (){
    let arr: [i32; 5] = [1,2,3,4,5];
    let tup: (i32, String, char)  = (1,String::from("feng"),'a'); 
    println!("{:?}", tup);
    println!("{},{},{}",arr[0], tup.1, tup.2);
}