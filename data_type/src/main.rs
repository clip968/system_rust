
fn main() {
    // let number = 30;
    // let long_number:i64 = 123456789123456789;
    // let real = 10.22;
    // let hangul_char = '러';

    // println!("32비트 정수 : {}", number);
    // println!("64비트 정수 : {}", long_number);
    // println!("32비트 실수 : {}", real);
    // println!("문자 : {}", hangul_char);

    // let tuple = (1, 2, 3);
    // println!("tuple : {}, {}, {}", tuple.0, tuple.1, tuple.2);

    // let (x, y, z) : (i32, char, bool) = (1, 'a', true);
    // println!("x={}, y={}, z={}", x, y, z);

    // let arr = [1, 2, 3, 4, 5];

    // for a in arr {
    //     print!("{}, ", a);
    // }

    // let arr: [i32; 5] = [1,2,3,4,5];

    // for i in 0..arr.len() {
    //     print!("{}, ", arr[i]);
    // }
    // println!("");

    // std::env::set_var("RUST_BACKTRACE", "1");

    // let mut arr = [1,2,3,4,5];

    // println!("숫자를 입력해주세요");
    // let mut read = String::new();
    // io::stdin().read_line(&mut read).unwrap();
    // let index:i32 = read.trim().parse().unwrap();
    // trim()은 \n 제거 ex: "hello\n" -> "hello"
    // parse()는 String에서 i32로 parsing, 즉 바꾸는 작업
    // unwrap()은 Result 형식에서 값을 꺼내는 것


    // println!("arr[{}] = {}", index, arr[index as usize]);

    // let s = "Hello 러스트!";

    // println!("문자열 : {}", s);

    // let slice = &s[0..5];
    // println!("슬라이스: {}", slice);

    // let s = " hello Rust ";
    // println!("{}", s.trim()); "hello Rust"
    // println!("{}", s.to_lowercase()); " hello rust "
    // println!("{}", s.to_uppercase()); " HELLO RUST "
    
    let mut s = String::from("Hello");
    println!("{}", s);
    s.push_str(" Rust");
    println!("{}", s);
    
}
