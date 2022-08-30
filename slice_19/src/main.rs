fn main() {
    let test = String::from("0123456");
    let mut test_slice = &test[0..5];
    println!("{}",test_slice);

    test_fun(&mut test_slice);
}


fn test_fun(s:&mut &str) -> String{
    let ss = s.replace("2", "1");
    ss
}