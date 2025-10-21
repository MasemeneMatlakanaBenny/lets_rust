//A program that calculates the user's age given that they emtered their birth year

#[allow(unused_variables)]

fn main(){
   //declare birth year and current year
    let birth_year : i32;
    let current_year: i32 =2025;
    
    //assuming that the user was born in 2018:
    birth_year = 2018;
    
    //compute the user's age
    let user_age = current_year-birth_year;
    println!("You are {} years old",user_age);
}
