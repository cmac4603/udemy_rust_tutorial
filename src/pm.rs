fn how_many(x: i32) -> &'static str {
    match x {
        0 => "no",
        1 | 2 => "one or two",
        12 => "a dozen",
        z @ 9...11 => "lots of", 
        _ if ( x % 2 == 0) => "some",
        _ => "a few",
    }
}

pub fn pattern_matching() {
    for x in 0..13 {
        println!("{}: I have {} oranges", x, how_many(x));
    }

    let point = (3,4);
    match point  {
        (0, 0) => println!("origin"),
        (0, y) => println!("x axis, y = {}", y),
        // create a ref to x for later use
        (ref x, 0) => println!("y axis, x = {}", x),
        // (x, y) => println!("({},{})", x, y),
        (_, y) => println!("(don't care about x,{})", y),
        // also you can use (x..) as example if we only care about one variable (here x)
    }
}