use std::fs;
use std::fs::File;
use std::io::Write;
use std::io::Error;

use crate::business::user::User;
use crate::business::user::UserManager;

// 사용자 목록을 파일에 저장

pub fn save(file_name: String, user_vec: Vec<&User>) -> Result<(), Error> {
    let mut buffer = File::create(file_name).expect("파일을 열 수 없습니다"); // 파일 생성
    for u in user_vec.iter() {
        let f = format!("{} {} {}\n", u.id, u.age, u.name); // 사용자 정보 포맷팅
        buffer.write(f.as_str().as_bytes())?; // 파일에 쓰기
        // 러스트에서 파일에 뭐 쓸때는 String이나 &str을 직접 전달을 못하기 때문에
        // 바이트 단위로 전달해줘야한다. 따라서 as_bytes로 파일에 글을 쓸 수 있다.
    }

    Ok(())
}

pub fn load(file_name: String) -> Vec<User> {
    let mut user_vec: Vec<User> = Vec::new(); //사용자 목록 저장용
    let txt = fs::read_to_string(file_name).expect("파일을 읽을 수 없습니다");


    // 각 줄을 분석
    for ln in txt.split("\n"){
        if ln.len() == 0 {
            break;
        }

        let tok: Vec<&str> = ln.split(" ").collect(); //공백으로 분리

        // 분리된 토큰으로 사용자 정보 생성
        user_vec.push(User {
            id: tok[0].parse::<i32>().unwrap(),
            age: tok[1].parse::<i32>().unwrap(),
            name: tok[2].to_string(),
        });
    }

    user_vec //사용자 벡터 반환
}



