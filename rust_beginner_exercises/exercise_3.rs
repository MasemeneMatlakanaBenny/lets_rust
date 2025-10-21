//Exercise 3:A Rust program that declares a boolean variable is_sunny and sets it to true if it's sunny and false otherwise:
fn main(){
    let is_sunny: bool=false; // it is not sunny for the day
    let cold_statement: &str="it is cold for the day where I am";
    let hot_statement: &str="it is hot for the day where I am";
    if is_sunny==true{
        println!("{}",hot_statement);
    }
    else{
        println!("{}",cold_statement);
    }
}
  
