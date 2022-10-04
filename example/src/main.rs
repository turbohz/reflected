use reflected::Reflected;
use reflected_proc::reflected;

#[reflected]
#[derive(Debug, Default)]
struct Data {
    string: String,
    int: i32,
}

fn main() {
    dbg!(Data::default());
    dbg!(Data::random());
    dbg!(Data::fields());

    println!("Hello, world!");
}
