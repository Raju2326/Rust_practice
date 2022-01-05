pub fn run (){
    let mut hello = String::from("Hello World   Murugaraju Perumalla");
    println!("hellow first {}",hello);
    hello.push('D');
    hello.push_str("ok ok");
    println!("hello after ----> {}",hello);
    let mut count=0;
    for character in hello.split_whitespace(){
        println!("{} count value {}",character,count);
        count+=1;
    }
    let mut s = String::with_capacity(10);
    s.push_str("A");
    s.push_str("B");
    println!("{}",s);
    assert_eq!(2,s.len());

}
