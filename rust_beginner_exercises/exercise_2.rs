// exercise 2 : lets calculate the area of the circle using the constant PI and given a diameter:
fn main(){
    let pi: f32= 3.142857142857142;
    let diameter: f32=4.0;
    let y : f32 = 2.0;
    let radius = diameter/y;
    
    //now lets compute the area of the circle:
    let area_circle=radius * radius * pi;
    println!("Using the first method,Area of Circle = {}",area_circle);
    
    //using the second method ,lets recall the x_squared created function and use it here:
    let area_circle_new=x_squared(radius)*pi;
    println!("Using the second method ,Area Of Circle = {}",area_circle_new);
    
    assert_eq!(area_circle,area_circle_new);
    println!("Area computed successfully using the two methods");
   }
fn x_squared(x: f32) -> f32{

    return x * x;
    
}
