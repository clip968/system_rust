// use std::io;
fn main() {
    // let var = 1;
    // println!("var = {}", var);
    // let var = var + 1;
    // println!("var = {}", var);
    // 기존의 var은 없어지고 var라는 새로운 변수가 생성됨 => shadowing

    // let var = 1;
    // println!("var = {}", var);
    // let var = String::from("기존 var을 섀도잉");
    // println!("var = {}", var);

    // let condition = true;
    // if condition == true { //condition이 true일 경우 작동
    //     println!("조건이 참입니다");
    // }
    // else {
    //     println!("조건이 거짓입니다.");
    // }

    // let condition = true;
    // let ret = if condition == true {
    //     String::from("조건이 참입니다")
    // } else {
    //     String::from("조건이 거짓입니다")
    // };

    // println!("ret = {}", ret);

    // let var = 4;
    // match var { 
    //     1 => println!("하나"),
    //     2 => println!("둘"),
    //     _ => println!("기타"),

    // } // switch와 비슷하지만 switch 보다 더 간편함.

    // let var = 1;
    // let ret = match var {
    //     1 => String::from("하나"),
    //     2 => String::from("둘"),
    //     _ => String::from("기타"),
    // };
    // println!("ret = {}", ret);

    // loop { // 다른 언어에서 while(true)와 같은 의미
    //     println!("숫자를 입력해주세요. 0을 입력하면 종료합니다");
    //     let mut read = String::new();
    //     io::stdin().read_line(&mut read).unwrap();
    //     let val:i32 = read.trim().parse().unwrap();

    //     if val == 0 {
    //         break;
    //     }

    //     println!("입력 = {}", val);
    // }
    
    // let arr = [1, 2, 3, 4, 5];
    // for a in arr {
    //     print!("{}, ", a);
    // }

    // for a in 0..5{
    //     print!("{}, ", a);
    // }

    // let mut counter = 0;
    // while counter < 5 {
    //     print!("{}, ", counter);
    //     counter += 1;
    // }

}
