pub fn run(){
    let mut numbers:[i32; 5] = [1,2,3,4,5];
    // Re-assign possiblities with Array
    numbers[2]=20;
    println!("printing the output---->{:?} and single slice value {}",numbers,numbers[3]);
    println!("Size of numbers array is {} bytes", std::mem::size_of_val(&numbers));
    // Get slices from an Array
    let slice:&[i32]=&numbers[1..4];
    println!("Slice data printing {:?}",slice);
}