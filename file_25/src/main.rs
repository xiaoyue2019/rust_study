use std::{fs::File,fs::{OpenOptions}, io::{Write, Read}};

fn main() {

    // let file = File::open("data2.txt").unwrap();
    // println!("文件打开:{:?}",file);

    // let file = File::create("data2.txt").expect("创建失败!");
    // println!("文件创建:{:?}",file);

    // let file = fs::remove_file("data2.txt").expect("删不了");
    // println!("文件删除:{:?}",file);

    let mut file = OpenOptions::new().append(true).open("data2.txt").expect("添加失败");
    file.write("asdf".as_bytes()).expect("写入失败");
    file.write_all("dfasdfasdfadsf".as_bytes()).expect("创建失败");


    let mut file = File::open("data2.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}",contents);
}
