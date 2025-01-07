use std::path::Path;
use std::os::unix::fs::PermissionsExt;
use std::fs::File;
use std::io::{self, Read, Write};
// use std::os::unix::io::{FromRawFd, IntoRawFd};
use std::env;
use std::process::Command;
use std::process;
fn main(){
    // let path = Path::new("test.txt");
    // // metadata는 파일크기, 권한 등 파일에 대한 다양한 정보를 담음
    // let metadata = path.metadata().unwrap();

    // //파일의 권한 정보를 불러옴
    // let permission = metadata.permissions();
    // let mode = permission.mode(); //권한을 8진수로 변환
    // println!("파일 접근 권한: {:o}", mode); //{:o}는 8진수로 출력하는 방법

    // let f = File::open("test.txt").unwrap();

    // let fd = f.into_raw_fd(); //f의 파일 서술자 획득

    // let mut f = unsafe {
    //     File::from_raw_fd(fd)
    // };

    // let mut contents = String::new();
    // f.read_to_string(&mut contents).expect("파일 읽기 실패");
    // println!("{}", contents);

    // let f = File::open("test.txt").unwrap();
    // let fd = f.into_raw_fd();

    // let mut f = unsafe {
    //     File::from_raw_fd(fd)
    // };

    // let mut contents = String::new();
    // f.read_to_string(&mut contents).expect("파일 읽기 실패");
    // println!("{}", contents);

    // 파일 디스크럽터란 운영체제가 파일이나 I/O자원을 식별하고 관리하는데 사용하는 추상적인 핸들이다.
    // 즉 운영체제는 파일을 직접 다루지 않고 파일 디스크럽터를 통해 파일을 간접적으로 참조한다.

    // for (index, argument) in env::args().enumerate() { 
    //     println!("인자 #{}: {}", index, argument);
    // }

    // env::set_var("my_env", "my_value");

    // match env::var("my_env") {
    //     Ok(value) => println!("my_env = {}", value),
    //     Err(e) => println!("my_env 읽기 오류 : {}", e),
    // }

    // env::remove_var("my_env");

    // match env::current_dir() {
    //     Ok(path) => println!("현재 경루 : {:?}", path),
    //     Err(e) => println!("현재 경로 획득 실패 : {}", e),
    // }

    // match env::temp_dir().to_str() {
    //     Some(path) => println!("임시 경로: {}", path),
    //     None => println!("임시 경로 확인 불가")
    // }

    // let echo = Command::new("echo")
    //     .arg("echo 실행") // echo 실행이라는 인자를 추가 -> echo "echo 실행"
    //     .output() // 명령어 실행 결과 저장
    //     .expect("echo 실행 실패");

    // let ret = String::from_utf8_lossy(&echo.stdout); 
    //만약 utf8값이 유효하지 않으면 손실(lossy)처리 되어 �(U+FFFD, 대체 문자)로 대체된다.

    // println!("결과 : {}", ret);

    // let pid = process::id();
    // println!("process ID {}", pid);

    // let mut file = File::create("example.txt");
    // file.write_all(b"Hello, Rust!")?;

}
