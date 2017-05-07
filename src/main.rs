#![allow(dead_code)]
#![allow(unused_variables)]
use std::mem;

enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8,u8,u8), // tuple
    CmykColor{cyan:u8, magenta:u8, yellow:u8, black:u8}, // struct
}

fn enums() {
    // let c:Color = Color::RgbColor(0,0,10);
    // let c:Color = Color::CmykColor{cyan: 0, magenta: 128, yellow: 0, black: 0};
    let c:Color = Color::CmykColor{cyan: 0, magenta: 128, yellow: 0, black: 255};

    match c {
        Color::Red => println!("red"),
        Color::Green => println!("green"),
        Color::Blue => println!("blue"),
        Color::RgbColor(0,0,0) 
        | Color::CmykColor{cyan:_,magenta:_,yellow:_,black:255}=> println!("black"),
        Color::RgbColor(r,g,b) => println!("rgh({},{},{})", r, g, b),
        _ => ()
    }
}

fn option() {
    // Option<T>

    let x = 3.0;
    let y = 0.0;

    // option can be Some(z) or None

    let result:Option<f64> =
        if y != 0.0 { Some(x/y) } else {None};

    println!("{:?}", result);

    match result {
        Some(z) => println!("{}/{} = {}", x, y , z),
        None => println!("Cannot divide {} by {}", x, y),
    }

    // if let / while let
    if let Some(z) = result { println!("z = {}", z); };
}

fn arrays() {
    let mut a:[i32;5] = [1,2,3,4,5];

    println!("a has {} elements, first is {}", a.len(), a[0]);

    a[0] = 321;
    println!("a[0] = {}", a[0]);

    println!("{:?}", a);

    if a != [321,2,3,4,5] {
        println!("does not match");
    }
    else {
        println!("does match!");
    }

    let b = [1u16;10]; // b.len() == 10
    for i in 0..b.len(){
        println!("{}", b[i]);
    }

    println!("b took up {} bytes", mem::size_of_val(&b));

    let mtx:[[f32;3];2] =
    [
        [1.0, 0.0, 0.0],
        [0.0, 2.0, 0.0]
    ];
    println!("{:?}", mtx);

    for i in 0..mtx.len() {
        for j in 0..mtx[i].len(){
            if i == j {
                println!("mtx[{}][{}] = {}", i, j, mtx[i][j]);
            }
        }
    }
}

fn vectors() {
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);

    println!("a = {:?}", a);

    a.push(44);
    println!("a = {:?}", a);
    
    let idx:usize = 0;
    a[idx] = 321;
    println!("a[0] = {:?}", a[idx]);

    // .get() on vector returns Option
    // match a.get(6) {
    match a.get(3) {
        Some(x) => println!("a[4] = {}", x),
        None => println!("error no such element"),
    }

    for x in &a {
        println!("{}", x);
    }

    a.push(77);
    println!("{:?}", a);

    let last_elem = a.pop(); // Option
    println!("{:?}", last_elem); // Some(x) if vector not empty
    println!("last_elem is {:?}, left with vector a = {:?}", last_elem, a); // Some(x) if vector not empty

    // let last_elem = a.pop(); // Option
    // let last_elem = a.pop(); // Option
    // let last_elem = a.pop(); // Option
    // let last_elem = a.pop(); // Option
    // let last_elem = a.pop(); // Option

    // match last_elem {
    //     Some(x) => println!("{}", x),
    //     None => println!("vector a is empty"),
    // }

    // how to iterate over Option returns,
    // this will fail when a.pop() returns None
    while let Some(x) = a.pop() {
        println!("In Some(x), x = {}", x);
    }
}

fn main () {
    //enums();
    //option();
    //arrays();
    vectors();
}
