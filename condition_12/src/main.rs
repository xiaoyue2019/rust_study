fn main() {
    let code = 10086;
    let _result = match code {
        10086 => "您是移动号码",
        10010 => "您是联通号码",
        _=>"Unknown"
    };
    println!("{}",_result);

    let total = 9;
    if total > 1 && total < 10{
        println!("大于1");
    }else if 10 > total{
        println!("大于1小于10");
    }
}
