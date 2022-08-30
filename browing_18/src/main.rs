fn main() {
    // 借用 fun(&args) 
    // 可变借用 fun(&mut args)  就是传入指针，和之前的引用传递很相似。
    // 感觉可以直接说，引用传递就是可变借用

    // let mut test = String::from("123");
    // test_fun(&mut test);
    // println!("{}",test);
}

// fn test_fun(t: &mut String){
//     t.push_str("da");
//     println!("{}",t);
// }