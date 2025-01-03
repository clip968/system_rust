// use tokio::fs::File;
use std::{fs::File, io::{BufRead, BufReader}};
// use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::Semaphore;
use serde::{Serialize, Deserialize};
use sqlite;
use sqlite::State;
#[tokio::main]
async fn main() {
    // 보통 하나의 자원은 하나의 스레드에서만 접근할 수 있도록 한다
    // 그러나 상황에 따라 하나의 자원을 동시에 공유해야 한다면? -> mutex, semaphore
    // 뮤텍스는 상호배제의 약자로 여러 스레드가 공유 자원에 동시에 접근하지 못하도록 하는 기법이다.
    // lock, unlock의 두 가지 상태가 존재한다. 
    // 만약 부주의로 unlock을 해제하지 않는다면 다른 스레드는 자원이 해제 될 때 까지 무한히 기다리는 경우 생김 => deadlock

    // let mut thread_vec = vec![];

    // // 100개의 스레드가 동시에 전역변수인 counter에 접근하는 것을 제어
    // for _ in 0..100 {
    //     let th = thread::spawn(inc_counter);
    //     thread_vec.push(th);
    // }

    // for th in thread_vec {
    //     th.join().unwrap();
    // }
    // // mutex값에 접근하려면 *을 사용하고 또한 무조건 lock을 걸고 접근해야 한다.
    // println!("결과: {}", *counter.lock().unwrap());
    
    // 세마포어는 복수의 제한된 자원에 다수의 스레드가 동시에 접근하는 것을 막는 동시성 제어 방법이다.
    // 세마포어는 복수의 자원을 관리하고 뮤텍스는 하나의 자원을 관리한다.
    // Arc<T>는 원자적 참조 카운트를 의미한다.
    // 즉 여러 스레드에 걸쳐 데이터의 소유권을 공유하는데 사용할 수 있다.

    // let semaphore = Arc::new(Semaphore::new(2));
    // let mut future_vec = vec![];
    
    // for _ in 0..100 {
    //     let permit = semaphore.clone().acquire_owned().await.unwrap();
    //     let future = tokio::spawn(async move {
    //         let mut num = counter.lock().unwrap();
    //         *num += 1;

    //         drop(permit);
    //     });
    //     future_vec.push(future);
    // }

    // for future in future_vec {
    //     future.await.unwrap(); // 모든 future가 완료될 때 까지 대기
    // }

    // println!("결과 : {}", *counter.lock().unwrap());

    // let counter = Arc::new(Mutex::new(0));
    // let mut thread_vec = vec![];

    // for _ in 0..100 {
    //     let _cnt = counter.clone(); // 현재 카운터의 클론 생성
    //     let th = thread::spawn(move || {
    //         let mut num = _cnt.lock().unwrap();

    //         *num += 1;
    //     });
        
    //     thread_vec.push(th);
    // }

    // for th in thread_vec {
    //     th.join().unwrap();
    // }

    // println!("결과: {}", *counter.lock().unwrap());

    // let lock_a = Arc::new(Mutex::new(0));
    // let lock_b = Arc::new(Mutex::new(0));

    // let lock_a_ref = lock_a.clone();
    // let lock_b_ref = lock_b.clone();

    // let thread1 = thread::spawn(move || {
    //     let a = lock_a.lock().unwrap();
    //     let b = lock_b_ref.lock().unwrap();
    // });

    // let thread2 = thread::spawn(move || {
    //     let b = lock_b.lock().unwrap();
    //     let a = lock_a_ref.lock().unwrap();
    // });

    // thread1.join().unwrap();
    // thread2.join().unwrap();

    // println!("프로그램 종료");

    // 동기식 입출력 예제
    // let mut file = File::open("input.txt").unwrap();
    // let mut contents = String::new();
    // file.read_to_string(&mut contents).unwrap();
    // 파일을 다 읽을 때 까지 대기
    // println!("{}", contents);

    // let mut file = File::create("output.txt").unwrap();
    // file.write_all(contents.as_bytes()).unwrap();
    // 파일을 쓸 때까지 대기

    // let mut file = File::open("input.txt").await.unwrap();

    // let mut contents = String::new();
    // file.read_to_string(&mut contents).await.unwrap();

    // println!("{}", contents);

    // let mut file = File::create("output.txt").await.unwrap();
    // file.write_all(contents.as_bytes()).await.unwrap();

    // let file = File::open("input.txt").unwrap();
    // let reader = BufReader::new(file);

    // for line in reader.lines() {
    //     let line = line.unwrap();
    //     println!("{}", line);
    // }

    // let pt = Point{x : 10, y : 10};

    // let json = serde_json::to_string(&pt).unwrap();
    // println!("json: {}", json);

    // let pt: Point = serde_json::from_str(&json).unwrap();
    // println!("point: [{} {}]", pt.x, pt.y);

    // 메모리에 sqlite db 생성
    let connection = sqlite::open(":memory:").unwrap();

    let query = "
    CREATE TABLE users (name TEXT, age INTEGER);
    INSERT INTO users VALUES ('루나', 3);
    INSERT INTO users VALUES ('러스트', 13);
    ";
    
    connection.execute(query).unwrap();

    let query = "SELECT * FROM users WHERE age > ?"; // ?는 1에 바인딩 됌
    let mut statement = connection.prepare(query).unwrap(); // 쿼리 실행
    statement.bind((1, 5)).unwrap(); // age > 5 
    
    while let Ok(State::Row) = statement.next() {
        println!("name = {}", statement.read::<String, _>("name").unwrap());
        println!("age = {}", statement.read::<i64, _>("age").unwrap());
    }
}

// static counter: Mutex<i32> = Mutex::new(0);

// fn inc_counter(){
//     let mut num = counter.lock().unwrap();
//     *num = *num + 1;
// } // inc_counter를 벗어나는 순간 counter는 unlock 된다.

// #[derive(Serialize, Deserialize)]
// struct Point {
//     x : i32,
//     y : i32,
// }


