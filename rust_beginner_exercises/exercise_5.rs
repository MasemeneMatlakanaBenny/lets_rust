//Exercise 5:A Rust program that creates an array of numbers containing 5 integers and we iterate over them
fn main{
    let arr_nums=[1,2,3,4,5] ;
    println!("An array of numbers in rust");
    
    //alternatively:
    let arr_nums_sec : [i32;5] = [1,2,3,4,5];
    //the first i32-> indicates the data type and the 5 indicates the length of the array:
    //lets iterate over the array;
    
    for &x in arr_nums.iter(){
        println!("{}",x);
    }

}
