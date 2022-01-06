// Enums are the types which have few definite values
enum Movement{
    Left,
    Up,
    Right,
    Down
}

fn move_avatar(m:Movement){
    match m {
        Movement::Left => println!("Avatar moving Left"),
        Movement::Up => println!("Avatar moving Up"),
        Movement::Right => println!("Avatar moving Right"),
        Movement::Down => println!("Avatar moving Down")

    }
}
pub fn run(){
let avatar1 = Movement::Left;
let avatar2 = Movement::Right;
let avatar3 = Movement::Up;
let avatar4 = Movement::Down;

move_avatar(avatar1);
}