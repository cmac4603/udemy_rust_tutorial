struct Person {
    name: String,
}

impl Person {
    // fn get_ref_name(&self) -> &String {
    fn get_ref_name<'a>(&'a self) -> &'a String {
        &self.name
    }
}

struct Company<'z> {
    name: String,
    ceo: &'z Person,
}

fn main() {
    // let boss = Person { name: String::from("Elon Musk"), };
    // let tesla = Company { name: String::from("Tesla"),
    //                       ceo: &boss, };

    {
        let p = Person { name: String::from("John") };
        let mut z = p.get_ref_name();
        println!("{:?}", z);
    }
}