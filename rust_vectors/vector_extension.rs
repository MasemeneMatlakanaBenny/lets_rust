//Assume we have two vectors A and B in our program.
//Say we want to extend vector A by vector B 
//Then we can simply do that by the use of extend method:

fn main(){
  let mut  vect_A : Vec<i32> =vec![1,2,3,4,5];
  let vect_B : Vec<i32>  =vec[6,7,8];
  println!("Vector A before extension method: {:?}",vect_A);
  vect_A.extend(vect_B);
  println!("Vector A after extension method: {:?}",vect_B);

}

  
