#[allow(unused_variables)]

//lets develop a simple function that takes in the user's two numbers and return sum and product:
fn main(){
   let (a,b)=(10,3);
   let x = sum(a ,b);
   
   let y=prod(a,b);
   
   let z= diff(a,b);
   
   println!("{}",z);
    
}

fn sum(x : i32, y: i32) -> i32{ //this is a function that returns the sum of two numbers -> 5+7=12
    return x +y ;
}

fn prod(x : i32, y: i32) -> i32{ //this is a function that returns the product of two numbers -> 5*7=35
    return x * y ;
}

fn abs_diff(x: i32,y:i32) -> i32{  //this is a function that returns simple absolute difference between two integers -> 5-7=2
    if  y > x{
        return y-x;
        
    }
    else{
        return x-y
    }
}
