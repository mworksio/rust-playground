use std::fmt;

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "({}, {})", self.0, self.1);
        writeln!(f, "({}, {})", self.2, self.3)
    }
}

fn main() {
    // A tuple is a collection of values of different types.
    let long_tuple = (1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true);
    println!("long_tuple first value is {}", long_tuple.0);
    println!("long_tuple second value is {}", long_tuple.1);

    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
    println!("tuple of tuples: {:?}",  tuple_of_tuples);

    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // why 13 elems
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("too long tuple: {:?}", too_long_tuple);

    let pair = (1, true);
    println!("pair is {:?}", pair);

    println!("the reversed pair is {:?}", reverse(pair));

    println!("one element tuple: {:?}", (1,));
    println!("one element tuple: {:?}", (5u32,));

    println!("just an integer: {:?}", (5u32));

    let tuple = (1, "Hello", 4.5, true);
    let (a, b, c, d) = tuple;
    println!("{}, {}, {}, {}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{}", matrix);
    println!("Debug: {:?}", matrix);
}

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (integer, boolean) = pair;
    (boolean, integer)
}

fn transpose(m: Matrix) -> Matrix {
    //TODO
    // let p = pair (m.1, m.2);
    // r = reverse(p);
    // m.1 = r.1;
    // m.2 = r.2;
    // t = m.1;
    // m.1 = m.2;
    // m.2 = t;
    // return m;
}