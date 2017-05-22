#![allow(unused_variables)]
fn main() {
    // value v used after pointer to data on heap
    //  moved from v to v2
    let v = vec![1,2,3];
    // let v2 = v;
    // println!("{:?}", v);

    // let foo = |v:Vec<i32>| ();
    // foo(v);

    // this is allowed however, because i32 are v.
    // small and not pointers like for vecs
    // here rust is making a copy
    let u = 1; // i32
    let u2 = u;
    println!("u = {}", u);

    // this does not work though, because now we are trying
    // to use a moved value 
    let u3 = Box::new(1); // i32 but with Box pointer
    // let u4 = u3;
    println!("u3 = {}", u3);

    // this uses a vec and returns, but is laborious
    // that is why borrowing exists
    let print_vector = |x:Vec<i32>| -> Vec<i32> {
        println!("{:?}", x);
        x
    };

    let vv = print_vector(v);
    println!("{}", vv[0]);
}