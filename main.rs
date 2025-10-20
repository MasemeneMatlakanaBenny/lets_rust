fn main(){
    
    let num1 =10;
    let num2=15;

    
    //lets build a simple rust program that allows us to switch between two variables:
    let num3=num2; //this is now equals to 15 ,hence we have stored num2 as 10:
    
    let num2=num1; //this now will be equal to 10
    
    let num1=num3; //this will then switch to 15
    
    assert_eq!(num1,15);
    assert_eq!(num2,10);
    
    println!("The two variables have been switched successfully");
    
}
