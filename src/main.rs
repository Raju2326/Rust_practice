use std::io;
mod types;
mod datatype_string;
mod datatype_tuple;
mod datatype_array;
fn main() {
    let mut check = String::new();

    println!("Please enter your input ---------------> ");
    io::stdin()
    .read_line(&mut check)
    .expect("readlines failed dude ----------------> ");
    println!("entering of the data ------------> {}",check);
    println!("practice {name}", name="makes a man perfect");
    types::run();

    datatype_string::run();
    datatype_tuple::run();
    datatype_array::run();
}
