use memmap::{MmapMut, MmapOptions};
extern crate libc;
use std::os::unix::io::AsRawFd;
use std::io;
use std::fs::OpenOptions;
use std::env;
use std::time::Duration;
use std::thread;

const FILE_PATH: &str = "shared_mmap_file.txt";

// 터미널 i/o 제어 명령어
const TIOCGWINSZ: libc::c_ulong = 0x5413;

#[repr(C)]
struct Winsize {
    ws_row: libc::c_ushort, // 문자 단위의 행수
    ws_col: libc::c_ushort, // 문자 단위의 열수
    ws_xpixel: libc::c_ushort, // 가로 픽셀 크기
    ws_ypixel: libc::c_ushort, // 세로 픽셀 크기
}

fn get_terminal_size() -> Result<(libc::c_ushort, libc::c_ushort), io::Error> {
    let ws = Winsize {
        ws_row:0,
        ws_col:0,
        ws_xpixel:0,
        ws_ypixel:0,
    };

    let fd = io::stdout().as_raw_fd();
    let result = unsafe {
        libc::ioctl(fd, TIOCGWINSZ, &ws)
    };

    if result == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok((ws.ws_col, ws.ws_row))
    }
}


fn main() {
    // let args: Vec<String> = env::args().collect();  

    // // cargo run write or cargo run read

    // if args.len() != 2 {
    //     println!("Usage: {} [write|read]", args[0]);
    //     return;
    // }

    // match args[1].as_str() {
    //     "write" => write_to_mmap(),
    //     "read" => read_from_mmap(),
    //     _ => println!("Use 'write' or 'read'."), // 유효하지 않은 인수 입력 시 출력
    // }

    match get_terminal_size() {
        Ok((width, height)) => {
            println!("Terminal Size: {}x{}", width, height);
        },
        Err(e) => {
            println!("Failed to get terminal Size: {}", e);
        }
    }

}

fn write_to_mmap() {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(FILE_PATH)
        .unwrap();

    let message = b"Hello from mmap!";
    file.set_len(message.len() as u64).unwrap();

    // 파일에 대한 mutable한 메모리 맵을 생성
    let mut mmap = unsafe {
        MmapOptions::new().map_mut(&file).unwrap()
    };

    // 메모리 맵에 메세지를 씀
    mmap.copy_from_slice(message);

    println!("메세지를 작성했습니다.");
}

fn read_from_mmap() {
    // mmap 오픈
    let file = OpenOptions::new().read(true).open(FILE_PATH).unwrap();
    let mmap = unsafe {
        MmapOptions::new().map(&file).unwrap()};

        let content = String::from_utf8_lossy(&mmap);
        println!("Read From mmap: {}", content);
    
}