fn main() {
    
    // let a = 1;
    let a = "asdf";

    // 变量可以隐藏、甚至修改类型隐藏「这其实就是更改了他的指针，使其指向另一块内存地址，抛弃销毁原来的变量」
    // 声明常量还有一种static的方式
    const PI:f64 = 3.1415926;

    static BOOK:&'static i32 = &1;

    println!("{},{},{}",PI,a,BOOK)
}
