fn main() {
    println!("Hello, world!");
    // 单个字符
    let a = '是';
    // 字符串
    let b = String::from("不是");
    // 字符串切片，或者说字符串字面量，他直接指向一个内存中不可变的地址
    let c = "是不是";

    // 这里使用深拷贝 将b的指针和值都拷贝下来 如果没有深拷贝 b的所有权就会转移到d 后面就不能用b了！！
    let d = b.clone() + "asdfasdf";

    println!("{},{},{},{}",a,b,c,d);
}
