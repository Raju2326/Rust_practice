pub fn run (){
    let age = 22;
    let check_id:bool=false;


    if age>=21{
        println!("what do you want?");
    }else{
        println!("you have to leave");
    }
    let is_of_age = if age <21 {true} else{false};
    println!("Is of age {}",is_of_age);
}