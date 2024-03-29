use std::mem;

fn main() {
    let var_u8: u8 = 0;
    let var_u16: u16 = 0;
    let var_u32: u32 = 0;
    let var_u64: u64 = 0;
    let var_i8: i8= 0;
    let var_i16: i16 = 0;
    let var_i32: i32 = 0;
    let var_i64: i64 = 0;
    let var_isize: isize = 0;
    let var_char: char = 'a';
    let var_float: f32 = 2.5;
    let var_double: f64 = 2.5;
    let var_boolean: bool = true;

    println!(" u8's value = {} , and its size is {}", var_u8, mem::size_of_val(&var_u8));
    println!(" u16's value = {} , and its size is {}", var_u16, mem::size_of_val(&var_u16));
    println!(" u32's value = {} , and its size is {}", var_u32, mem::size_of_val(&var_u32));
    println!(" u64's value = {} , and its size is {}", var_u64, mem::size_of_val(&var_u64));
    println!(" i8's value = {} , and its size is {}", var_i8, mem::size_of_val(&var_i8));
    println!(" i16's value = {} , and its size is {}", var_i16, mem::size_of_val(&var_i16));
    println!(" i32's value = {} , and its size is {}", var_i32, mem::size_of_val(&var_i32));
    println!(" i64's value = {} , and its size is {}", var_i64, mem::size_of_val(&var_i64));
    println!(" isize's value = {} , and its size is {}", var_isize, mem::size_of_val(&var_isize));
    println!(" char's value = {} , and its size is {}", var_char, mem::size_of_val(&var_char));
    println!(" f32's value = {} , and its size is {}", var_float, mem::size_of_val(&var_float));
    println!(" f64's value = {} , and its size is {}", var_double, mem::size_of_val(&var_double));
    println!(" boolean's value = {} , and its size is {}", var_boolean, mem::size_of_val(&var_boolean));
}
