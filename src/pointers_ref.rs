//  Reference pointers - Point to resource in Memory

pub fn run(){
    // Its Primitive Array
    let arr1 = [1,2,3];
    let arr2=arr1;


    println!("Array1 {:?} Array2 {:?}", arr1,arr2);

    //With non-primitive data type, if the data value assigned to another variable, then next variable no longer hold
    // the data, you will need to use reference to keep hold of second literal

    let a1: Vec<u32>= vec![1,3];
    let a2=&a1;
    println!("printing the a1 {:?} and a2 {:?}", a1,a2 );
}