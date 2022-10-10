use reflected::Reflected;
use reflected_proc::reflected;

#[reflected]
#[derive(Debug, Default)]
struct Data {
    string: String,
    int: i32,
}

#[reflected]
#[derive(Debug, Default)]
struct HasData {
    top_stre: String,
    top_int: i32,
    data: Data,
}

fn main() {
    dbg!(Data::default());
    dbg!(Data::random());
    dbg!(Data::fields());
    dbg!(Data::FIELDS);

    dbg!(HasData::FIELDS);

    println!("Hello, world!");
}
