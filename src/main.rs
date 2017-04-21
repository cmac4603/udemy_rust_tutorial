#![allow(dead_code)]
#![allow(unused_variables)]

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

fn main () {
    //enums();
    option();
}
