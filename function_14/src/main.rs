fn main() {
    let test_string = test();
    println!("{}",test_string);

    // let test_String = String::from("测试字符串");
    let test_int = 1;
    test2(test_int);
    println!("{}",test_int);

    // 传入可变引用类型，也就是引用传递
    // 主要搞懂的就是值传递和引用传递的方式。  function(args)   ||   function(&args) or function(&mut args)
    // &符号取内存地址 *符号取内存地址里的值
    let mut cc = 1;
    test3(&mut cc);
    println!("{}",cc);
}

fn test3(icc:&mut i32){
    println!("{}",*icc);
}
fn test2(mut a:i32){
    a = a*3;
    println!("{}",a);
}

fn test() -> String{
    // 最后一条语句一定不能有分号
    println!("123");
    String::from("456")
}