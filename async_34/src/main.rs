use std::{time::Duration, thread};

use futures::executor::block_on;

fn main() {
    block_on(testm());
}

async fn testm(){
    futures::join!(fun(),fun2());
}

async fn fun(){
    // fun2().await;
    thread::sleep(Duration::from_millis(2000));
    println!("go!");
}

async fn fun2(){
    println!("no go!")
}