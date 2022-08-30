fn main() {
    // 转让所有权 直接赋值，函数传递，函数返回 这三个都会发生所有权的转移
    // let test = String::from("dd");
    // let test = vec!["dd"];
    // let test = 1;
    // let test1 = test;
    // println!("{:?}",test);

    let test = String::from("dd");
    let test1 = fun_test1(test);
    fun_test(test1);
    println!("{:?}",test1);
}

fn fun_test(v:()){
    println!("{:?}",v);
}

fn fun_test1(v:String){
    println!("{}",v)
}