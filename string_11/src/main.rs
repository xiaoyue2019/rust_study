fn main() {
    // 两种字符串表示方式
    let _str = "我是字符串字面量";
    let _string = String::from("我是字符串结构体，或者叫字符串对象");
    println!("{},{}",_str,_string);

    // len、push_str、replace、to_string、as_str、trim、split、chars
    let mut a = String::new();
    println!("{}",a.len());

    a.push_str("aa");
    println!("{}",a);

    let b = a.replace("a", "b");
    println!("{}",b);

    let a = "a";
    let b = a.to_string();
    let c = b.as_str();
    println!("{}",c);

    let a = "\rfd,sdf,sfsfsf\n";
    for i in a.trim().split(","){
        for j in i.chars(){
            println!("{}",j);
        }
    }
}
