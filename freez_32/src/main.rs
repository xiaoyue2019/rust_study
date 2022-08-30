fn main() {
    let a = String::from("123");
    let b = &a;

    // 被借用着的变量不能再改动
    // a.push_str("123");
    println!("{},{}",b,a);
}
