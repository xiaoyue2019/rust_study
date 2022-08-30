use std::fmt::Display;

fn main() {
    // 泛型集合、枚举、函数 特性

    let mut generic_vecotor:Vec<f64>=Vec::new();
    generic_vecotor.push(123.11);
    println!("{:?}",generic_vecotor);

    let pcl = Person{
        name:String::from("123")
    };

    // let pcl=Person{
    //     name:1
    // };

    println!("{}",pcl);
    show(pcl);
}   

struct Person<T>{
    name:T
}

impl Display for Person<String> {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        println!("my name is {}",&self.name);
        let r = Result::Ok(());
        r
    }
}

fn show<T:Display>(t:T){
    println!("{}",t);
}