fn main() {
    tuples()
}

fn tuples() {
    let _my_tup: (i32, String, bool) = (42, String::from("hello, world"), true);
    println!("{}", _my_tup.1);
}
