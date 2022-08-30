fn main() {
    let mut pcl = Person{
        name: String::from("蒲沧龙"),
        age: 22,
        fix: String::from("我会很多东西")
    };

    println!("{:?}",pcl);
    pcl.age = 123;
    println!("name is : {} age is : {} fix is : {}",pcl.name,pcl.age,pcl.fix);
    // show(pcl);
    println!("{:?}",return_in(String::from("蒲沧龙")));
    println!("{}",pcl.get_age());
    println!("{:?}",Person::make_person(String::from("name")));
}

#[derive(Debug)]
struct Person{
    name:String,
    age:i32,
    fix:String
}

// fn show(s:Person){
//     println!("{:?}",s);
// }

fn return_in(name:String) -> Person{
    return Person{
        name:name,
        age:99,
        fix:String::from("adsf")
    }
}

impl Person {
    fn get_age(&self)->i32{
        return self.age;
    }

    fn make_person(name:String)->Person{
        return Person { name: name, age: 1998, fix: String::from("dasdfasdfads") };
    }
}