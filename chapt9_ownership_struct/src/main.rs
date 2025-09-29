struct Counter{
    number:i32,
}

impl Counter{
    fn new(number:i32)->Self{
        Self{number}
    }
    // Bukebianjieyong
    fn get_number(&self)->i32{
        self.number
      
    }

    fn add(&mut self , increment:i32){
        self.number + increment;
    }
}


fn main ()
{
    let counter = Counter{
        number:10
    };
    
}