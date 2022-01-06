pub fn run (){
    let mut numbers:Vec<i32> = vec![1,2];
    numbers.push(3);
    numbers.push(5);
    println!("{:?}",numbers);
    let slice: &[i32] = &numbers[1..3];
    println!("printing the slice of vector ---> {:?}",slice);
    for x in numbers.iter_mut(){
        *x*=2;
        println!("printing the number {}",x);
    }
}