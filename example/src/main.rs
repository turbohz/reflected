use reflected::Reflected;
use reflected_proc::reflected;

#[reflected]
#[derive(Default, Debug)]
struct Data {
    _f32: f32,
    _f64: f64,

    _i32: i32,
    _u32: u32,
    _i64: i64,
    _u64: u64,

    _isize: isize,
    _usize: usize,

    _string: String,
}

fn main() {
    dbg!(Data::FIELDS);

    assert_eq!(dbg!(Data::fields().len()), 9);
}
