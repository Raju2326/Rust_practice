pub fn run(){
    let mut count=0;

    loop {
        count +=1;
        println!("Count --- > {}",count);
        if count==20{
            break;
        }
    }
    // Fizz buzz While loop

    while count <=100 {
        if count % 15==0{
            println!("fizzbuzz")
        }else if count% 3==0{
            println!("fizz")
        }else if count% 5 == 0{
            println!("buzz")
        }else{
            println!("{}",count)

        }
        count+=1;
    }

    // For Range
    for x in 0..100{
        println!("printing x --->{}",x);
    }
}