fn main() {
    // iter into_iter iter_mut
    let mut _list = vec![1,2,3];
    for i in _list.iter(){
        println!("{}",i);
    }

    // 这样是会被移动所有权的
    // for i in _list{
    //     println!("{}",i);
    // }

    // 这里通过match匹配到list中的数据然后进行了更改
    for i in _list.iter_mut(){
        *i = match i {
            &mut 1 => {
                123123
            }
            _=>*i
        }
    }

    // while和loop就不说了吧
    let mut num = 1;
    while num < 10 {
        println!("{}",num);
        num=num*2;
    }

    let mut num = 1;
    loop {
        println!("{}",num);
        if num > 10{
            break;
        }
        num=num*3
    }

    println!("{:?}",_list);
    
}
