//A Rust program that will use constants to see how they work

fn main(
    //Force=ma ,a =9.8
    let mass : f32=10.0;
    const ACC: f32=9.8;
    let force= mass * ACC;
    println!("Force= {} N",force);
}
