//  Struct is used to define custom data type, we can think of this as Class related other programm
// Traditional struct
struct Color{
    red:u8,
    green:u8,
    blue:u8
}
// Tuple struct
struct ColorT(u8,u8,u8);

// Person struct
struct Person{
    first_name: String,
    last_name: String

}
impl Person{
    // Construct Person
    fn new(first: &str, last: &str)->Person{
        Person{
            first_name:first.to_string(),
            last_name:last.to_string()
        }
    }

    fn fullname(&self)-> String{
        format!("{} {}",self.first_name ,self.last_name)
        // self.first_name+&self.last_name

    }
    fn set_lastname(&mut self, name: &str){
        self.last_name=name.to_string();

    }
}

pub fn run() {
    let mut s = Color{
        red:255,
        green:0,
        blue:0
    };
    s.green=23;
    let s_tup=ColorT(244,0,3);

println!("printing the Struct Color {} {} {}", s.red, s.green, s.blue);
println!("printing the Struct Color Tuple {} {} {}", s_tup.0, s_tup.1, s_tup.2);

// Person related object writing here
let mut p = Person::new("Murugaraju","Perumalla");
println!("Person {} {}, {:?}",p.first_name,p.last_name,p.fullname());
// setting different last name
p.set_lastname("Rajkumar");
println!("Fullname {}",p.fullname());


}