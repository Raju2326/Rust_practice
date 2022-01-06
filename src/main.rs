use std::io;
mod types;
mod datatype_string;
mod datatype_tuple;
mod datatype_array;
mod datatype_vectors;
mod conditionals;
mod loops;
mod functions;
mod pointers_ref;
mod structs;
mod enums;
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
    datatype_vectors::run();
    conditionals::run();
    loops::run();
    functions::run();
    pointers_ref::run();
    structs::run();
    enums::run();
}
