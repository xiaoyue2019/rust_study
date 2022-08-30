use std::io::{stdin, stdout, Write};

fn main() {
    print!("shell>");
    stdout().flush().unwrap();

    let mut my_word = String::new();
    stdin().read_line(&mut my_word).unwrap();
    stdout().write(my_word.as_bytes()).unwrap();

    for i in std::env::args(){
        println!("参数分别是：{}",i);
    }
}
