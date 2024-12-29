use std::{f32::consts::E, fs::File, io::{self, Read}};
fn main() {
    // let result = File::open("test.txt");

    // let f = match result {
    //     Ok(f) => f,
    //     Err(err) => {
    //         panic!("파일 열기 실패: {:?}", err)
    //     },
    // };

    // println!("파일 열기 성공");

    // unwrap은 Result가 OK(T)일 경우 T의 값을 반환함
    // let f = File::open("./test.txt").unwrap();

    // expect는 Result가 Err(E)일 경우 지정된 오류 메세지를 출력한다
    // 그리고 unwrap과 마찬가지로 OK일 경우 T의 값을 반환
    // let f = File::open("test.txt").expect("에러");

    // println!("파일 열기 성공");

    let ret = read_from_file(String::from("test.txt")).expect("파일 읽기 중 오류가 발생하였습니다");

    println!("test.txt : {}", ret);
}

// fn read_from_file(path: String) -> Result<String, io::Error> {
//     let mut s = String::new();
//     let mut f = match File::open(path) {
//         Ok(f) => f,
//         Err(e) => return Err(e),
//     };

//     match f.read_to_string(&mut s) {
//         Ok(_len) => return Ok(s),
//         Err(e) => return Err(e),
//     };
// }

fn read_from_file(path:String) -> Result<String, io::Error> {
    let mut s = String::new();
    let mut f = File::open(path)?;
    let _ret = f.read_to_string(&mut s)?;
    Ok(s)
}