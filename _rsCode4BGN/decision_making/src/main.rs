fn main() {
    println!("Hello, world!");
    let my_bool = true;
    if my_bool == true {
        println!("hello");
    } else {
        println!("goodbye");
    }

    let n = 8;
    if n > 5 {
        println!(">5");
    } else if n < 5 {
        println!("<5");
    } else {
        println!("=5");
    }

    let some_bool = true;
    match some_bool {
        true => println!("its true"),
        false => println!("its false"),
    }

    
}
