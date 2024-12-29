// fn longest(x:&str, y:&str) -> &str {
//     // x, y, 반환 타입 모두 소멸 시점이 명확히 드러나지 않아 컴파일 오류 발생
//     if x.len() > y.len() {
//         x
//     }
//     else {
//         y
//     }
// }

// fn longest<'a>(x:&'a str, y:&'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }
static GLOBAL_CONST:i32 = 10;
fn main() {
    // 라이프타임 지시자는 변수를 대여할 때 대여 기간을 명시적으로 지정하기 위해 사용한다
    // let s1 = String::from("Hello");
    // let s2 = String::from("Rust");

    // let result = longest(&s1, &s2);
    // println!("{}와 {} 중 더 긴 문자열은 '{}'", s1, s2, result);

    // 정적 변수는 프로그램이 종료될 때 까지 메모리에 유지되는 변수
    // static GLOBAL_CONST: i32 = 10; 이런식으로 만든다.

    let x: &'static str = "Hello Rust";
    println!("x: {}", x);
    println!("Global_const: {}", GLOBAL_CONST);


}
