#![allow(unused_variables)]
#![allow(dead_code)]
mod sh;
mod if_exp;
use std::mem;

const MEANING_OF_LIFE:i8 = 42; // no fixed address

static mut Z_TWO:i32 = 132;

fn fundamental_data_types() {
    
    // i8 u8 i16 u16 i32 u32
    let a:u8 = 123; // 8bits 0..255
    let  mut b:i8 = 0; // 8bits -255..255 and mutable
    println!("a = {}, b = {}", a, b);

    b = 42;
    println!("a = {}, b = {}", a, b);

    let mut c = 123456789;
    println!("c = {}, size = {} bytes", c, mem::size_of_val((&c)));
    c = -1;
    println!("c = {} after mod", c);

    let z:isize = 123; // isize/usize
    let size_of_z = mem::size_of_val(&z);
    println!("z = {}, takes up {} bytes, {}-bit OS",
      z,size_of_z, size_of_z*8);

    let d:char = 'x';
    println!("d = {}, size = {} bytes", d, mem::size_of_val((&d)));

    let e:f32 = 2.5; // double-precision value, 8bytes or 64 bits, f64
    println!("e = {}, size = {} bytes", e, mem::size_of_val((&e)));

    // true or false
    let g = false;
    println!("g = {}, size = {} bytes", g, mem::size_of_val((&g)));

    let f = 4>0; //true
    println!("f = {}, size = {} bytes", f, mem::size_of_val((&f)));
}

fn operators() {
    // arithmetic
    let mut a = 2+3*4; //+-*/
    println!("a = {}", a);
    a = a+1;
    a -= 2; // -= += /= %=
    println!("remainder of {} / {} = {}", a, 3, (a%3));
    let a_cubed = i32::pow(a, 3);
    println!("{} cubed is {}", a, a_cubed);

    let b = 2.5;
    // integral power
    let b_cubed = f64::powi(b, 3);
    println!("\n{} cubed is {}", b, b_cubed);
    // float power
    let b_to_pi = f64::powf(b, std::f64::consts::PI);
    println!("{} to the power of pi = {}", b, b_to_pi);

    //bitwise
    let c = 1 | 2; // | OR & AND ^ XOR ! NOR
                   // 01 OR 10 = 11 == 3_10
    println!("1|2 = {}", c);

    let two_to_10 = 1 << 10; // >>
    println!("2^10 = {}", two_to_10);

    // logical
    let pi_less_4 = std::f64::consts::PI < 4.0; //true
    // < > <= >= ==
    let x = 5;
    let x_is_5 = x == 5; // true
    println!("is pi less than 4 -- {}", pi_less_4);
    println!("x is equal to 5 -- {}", x_is_5);
}

fn scope_and_shadowing() {
    let a = 123;
    let a = 777;
    {
        let b = 456;
        println!("inside, b = {}", b);
        let a = 888;
        println!("inside, a = {}", a);
    }
    println!("outside a = {}", a);
}

fn main() {
    // fundamental_data_types();
    // operators();
    // scope_and_shadowing();
    //unsafe
    //{
    //    Z_TWO = 125;
    //    println!("{}", Z_TWO);
    //}
    // println!("{}", Z_TWO);
    //sh::stack_and_heap();
    if_exp::if_statement();
}
