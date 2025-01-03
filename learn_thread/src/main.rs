use std::{fs::File, io::{BufRead, BufReader}, sync::mpsc, thread, time::Duration};
use futures::executor::block_on;
use tokio::time;
use std::io;
#[tokio::main]
async fn main() {
    // 새로운 스레드를 생성하고 실행
    // let handle = thread::spawn(|| {
    //     println!("쓰레드에서 실행");
    // });

    // 스레드가 완료 될때 까지 대기(종료 대기)
    // handle.join().unwrap();

    // let handle = thread::spawn(|| { // 새로운 스레드를 생성하고 그 핸들을 받는다
    //     let file = File::open("file.txt").unwrap();
    //     let reader = BufReader::new(file);
    //     for line in reader.lines() { // 파일의 각 줄을 순회
    //         let txt = line.unwrap(); // 해당 줄을 텍스트로 읽는다.
    //         println!("{}", txt);
    //     }
    // });

    // handle.join().unwrap(); //스레드가 끝날 때까지 기다린다.
    // match handle.join() {
    //     Ok(_) => {},
    //     Err(e) => {
    //         println!("스레드 내부에서 오류가 발생했습니다. {:?}", e);
    //     }
    // };

    //러스트는 스레드 간의 소유권을 양도받을 수 없음
    //따라서 데이터를 공유하려면 채널을 사용하여야 한다
    //mpsc = multiple producer, single consumer의 약자
    //송신자는 복수가 될 수 있으나 소비자는 단 하나만 존재한다.
    // let (tx, rx) = mpsc::channel();

    // thread::spawn(move || {
    //     let mut sum = 0;
        
    //     for i in 1..101 {
    //         sum = sum + i;
    //     }
    // 계산된 합을 채널로 보내기
    //     tx.send(sum).unwrap();
    // });
    // 메세지 수신
    // let sum = rx.recv().unwrap();
    // println!("1부터 100까지의 합: {}", sum);

    // let (tx1, rx) = mpsc::channel();
    // let tx2 = mpsc::Sender::clone(&tx1);

    // thread::spawn(move || {
    //     let mut sum = 0;
    //     for i in 1..51 {
    //         sum += i;
    //     }

    //     tx1.send(sum).unwrap();
    // });

    // thread::spawn(move || {
    //     let mut sum = 0;
    //     for i in 51..101 {
    //         sum += i;
    //     }

    //     tx2.send(sum).unwrap();
    // });

    // let mut sum = 0;
    // for val in rx {
    //     println!("수신: {}", val);
    //     sum += val;
    // }

    // println!("1부터 100까지의 합: {}", sum);

    // let future = hello_world();
    // println!("main 함수에서 실행");

    // // future 실행. hello_world가 끝날 때 까지 main thread는 멈춘다.
    // block_on(future);
    // println!("future 종료 이후 실행");
    
    // let future = calc();
    // let sum = calc().await;

    // 이벤트 루프는 애플리케이션이 구동되는 동안 사용자 이벤트나 gui작업 등의 이벤트를 처리하는데 사용함
    println!("아무 내용이나 입력하세요. quit를 입력하면 종료됩니다.");

    loop {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();

        let input = buf.trim();
        if input == "quit" {
            break;
        }
        println!("입력: {}", input);
    }
}
// async fn hello_world() {
//     println!("future 비동기 안에서 실행");
// }

async fn calc_sum(start: i32, end: i32) -> i32 {
    let mut sum = 0;

    for i in start..=end {
        print!("{}", i);
        sum += i;
    }

    sum
}

async fn calc() -> i32 {
    let f1 = sleep_10sec();
    let f2 = calc_sum(1, 10);
    // 본래 기대한건 f1, f2가 같이 실행되는건데 f1이 먼저 실행하여 모든 스레드가 작동을 멈춤(sleep)
    let(_, sum) = tokio::join!(f1, f2);

    sum
}

async fn sleep_10sec(){
    for i in 1..10 {
        print!(".");
        time::sleep(Duration::from_millis(1000)).await; // 1초간 10회 대기
    }
}