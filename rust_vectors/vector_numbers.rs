//lets create a vector filled with a certain number only
//if you are from python,we know that we can use numpy to create a vector or a matrix full of ones only 

//Here is the python  code:
// import numpy as np
// mat_ones=np.ones(1,5) -> this will then create a matrix of size 5 with  ones 

//Now lets create a vector of ones in Rust:

fn main(){
  let vector_omes : Vec<i32> = vec![1;5];
  println!("A vector filled with ones in Rust {:?}",vector_ones);

  //lets create a vector of zeros
  let vector_zeros: Vec<i32> = vec![0;5];
  println!("A vector filled wth zeros in Rust {:?}",vector_zeros);

  //lets create a vector of only 10s of size 4:
  let vector_tens : Vec<i32> = vec![10,4];
  println!("A vector filled with tens in Rust {:?}",vector_tens);

}
