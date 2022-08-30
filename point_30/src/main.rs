use std::ops::Deref;

fn main() {
    let test1 = 1;
    let test2 = Box::new(test1);
    println!("{}",*test2 == 1);


    let mb = Mybox{value:1};
    println!("{}",mb.value==*mb);
}

struct Mybox<T>{
    value:T
}

impl<T> Deref for Mybox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> Drop for Mybox<T> {
    fn drop(&mut self) {
        println!("解除")
    }
}