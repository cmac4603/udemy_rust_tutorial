#![allow(dead_code)]
#![allow(unused_variables)]
// Option<T> is for defining types
// below is how one can define multiple types in a struct
// struct Point<T, V> {
//     x:T,
//     y:V,
// }

struct Point<T> {
    x: T,
    y: T,
}

struct Line<T> {
    start: Point<T>,
    end: Point<T>,
}

fn generics() {
    // let a:Point<u16, i32> = Point { x: 0.3, y: 4 };
    // let b:Point = Point { x: 1.2, y: 3.4};
    let a:Point<f64> = Point { x: 0.0, y: 4f64 };
    let b = Point { x: 1.2, y: 3.4};

    let myline = Line { start: a, end: b};
}

fn print_value(x: i32) {
    println!("value = {}", x);
}

fn increase(x: &mut i32){
    *x += 1;
}

fn product(x: i32, y:i32) -> i32 {
    x * y
}

fn functions() {
    print_value(33);

    let mut z = 1;
    increase(&mut z);
    println!("z = {}", z);

    let a = 3;
    let b = 5;
    let p = product(3, 5);
    println!("{} * {} = {}", a, b, p);
}

fn main() {
    // generics();
    functions();
}