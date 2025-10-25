//A Rust program that add elements to an initialized Vector :
//lets create an empty vector in which we can add elements of our choice:

fn main(){
    let mut vec_new : Vec<i32> = Vec::new();  //make sure that the vector created is mutable first:
    vec_new.push(10);
    vec_new.push(20);
    vec_new.push(30);
    println!("The vector created filled with elements of our choices : {:?}",vec_new);
    println!("The vector created has {} elements",vec_new.len());
}
