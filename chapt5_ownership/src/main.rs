fn getlength(s:String) -> usize{
    println!("String:{}",s);
    s.len() 
}//对于传入这个函数的参数，函数结束后，参数会被销毁

//返回一个字符串
fn dangle()->String{
    "hello".to_string()
}

//静态指针的生命周期
fn dangle_static()-> &'static str{
    "hello"
}
//String 与 &str
fn first_word(s:&str)-> &str{
    let bytes: &[u8] = s.as_bytes();
    for (i,&item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
fn main (){
    //copy
    let c1: i32 = 1 ;
    let c2: i32  = c1;
    println!("{}",c1);
    println!("{:?}",c2);

    //move
    let s1: String = String::from("value");
    let s2: String = s1.clone();
    // println!("{:?}",s1);
    println!("{:?}",s2);
    println!("{}",s1);

    //func
    let len: usize = getlength(s1);
    println!("{}",len);


}
