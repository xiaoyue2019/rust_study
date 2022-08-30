fn main() {
    // 可恢复错误和不可恢复错误
    // panic!("adsf");
    println!("123");
    println!("{}",test_error(1).unwrap());
}

fn test_error(i:i32) -> Result<bool,String>{
    if i > 10{
        Ok(true)
    }else {
        Err("输入错误！".to_string())
    }
}