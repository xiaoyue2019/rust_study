fn main() {
    println!("Hello, world!");
    // 访问元组元素
    let test_tuple = (53,13);
    println!("{:?}",test_tuple);
    println!("{}",test_tuple.0);

    // 元组作为餐宿
    fn_test_tuple(test_tuple);


    // 元组解构
    let (first,second) = test_tuple;
    println!("{},{}",first,second);
}

fn fn_test_tuple(tuple:(i32, i32)){
    println!("{:?}",tuple);
}