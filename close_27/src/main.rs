fn main() {
    let close = |x|{x*2};
    let i = 2;
    let close2 = |a,b|{a+b-2+i};
    println!("{}",close(2));
    println!("{}",close2(1,2))
}
