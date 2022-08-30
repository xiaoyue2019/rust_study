use std::{collections::HashMap};

fn main() {
    // hashmap，HashSet，vector
    let mut bedroom = vec!["蒲沧龙","肖越","肖越二号"];
    bedroom.push("肖越三号");
    bedroom.remove(0);
    match bedroom.contains(&"肖越") {
        true=>println!("存在"),
        false=>println!("不存在")
    }
    println!("{:?},{}",bedroom,bedroom.len());

    let mut bedroom = HashMap::new();
    bedroom.insert(0, "蒲沧龙");
    bedroom.insert(1, "肖越");
    // bedroom.remove(&0);
    for (k,v) in bedroom.iter(){
        println!("{},{}",k,v);
    }
    match bedroom.get(&0) {
        Some(d)=>println!("{}",d),
        None=>println!("没找到")
    }

    
}
