fn main() {

    let mut array_test = [1,2,3,4];
    let array_test_default = ["";3];
    println!("{:?},{:?}",array_test,array_test_default);

    println!("{}",array_test.len());

    for i in array_test{
        println!("{}",i);
    }

    array_test[1] = 1;

    value_func(array_test);
    println!("{:?}",array_test);

    qut_func(&mut array_test);
    println!("{:?}",array_test);
}

fn value_func(mut arr:[i32;4]){
    println!("{:?}",arr);
    arr[1]=100;
}

fn qut_func(arr:&mut [i32;4]){
    println!("{:?}",arr);
    arr[1]=100;
}