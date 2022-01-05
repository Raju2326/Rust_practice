pub fn run (){

    let person: (&str, &str, i16)= ("Murugaraju", "Perumalla", 30);
    println!("{:?}", person);
    println!("{} {} {}",person.0, person.1, person.2);
}