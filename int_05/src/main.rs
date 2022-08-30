use std::i32::MAX;
use std::i64::MAX as im;

fn main() {
    let a:i32 = -1;
    let b:u32 = 1;
    let c = 1;
    let d:i8 = 127;

    // 这里编译阶段就报恐慌了 添加溢出
    // println!("{},{},{},{}",a,b,c,d+1);
    println!("{},{},{},{}",a,b,c,d);

    println!("i32最大值是:{}",MAX);
    println!("i64最大值是:{}",im);
}
