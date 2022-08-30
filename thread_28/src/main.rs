use std::{thread, time::Duration};

fn main() {

    for i in 1..5{
        println!("{}",i);
        thread::sleep(Duration::from_millis(1));
    }

    let thread1 = thread::spawn(||{
        for i in 1..10{
            println!("{}",i);
            thread::sleep(Duration::from_millis(1));
        }}
    );

    thread1.join().unwrap();
}