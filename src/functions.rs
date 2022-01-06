pub fn run (){
 greeting("Hello","Murugaraju");
//  Bind values to x variable
 let x=add(5,3);
 println!("add value {}",x);
//  Closure related need to know 
// shortform writing of functions as well the best benifit comes from usage of outside variables
let n3:u32=10;
//outside variable above n3
let sum = |n1: u32, n2: u32|n1+n2+n3;
println!("printing add via Closure {}", sum(5,6));
}

fn greeting(greet: &str, name: &str){
    println!("{} {}, nice to meet you",greet,name);
}

fn add (num1: u32, num2: u32 ) -> u32{
    num1+num2
}