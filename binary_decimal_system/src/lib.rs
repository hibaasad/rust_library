pub fn binary_to_decimal(input:i32){
    let binary:String=input.to_string();
    let rev=decimal.chars().rev().collect::<String>();
    let mut power:u32=0;
    let mut number:u32=2;
    let mut total:u32=0;
    for i in rev.chars(){
        let iterate=(i.to_string()).parse::<i32>().unwrap();
        if iterate=1{
            total=total+number.pow(power);

        }
        power=power+1
    }
    println!("{}",total);

}

