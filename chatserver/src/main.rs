use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tokio::sync::broadcast;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    // localhost의 1234번 포트를 할당
    let listener = TcpListener::bind("localhost:1234").await?; // localhost의 1234번 포트를 할당
    let (tx, _) = broadcast::channel(10); // 스레드 간 통신할 채널을 생성
    let shared_tx = Arc::new(tx); // 다른 스레드에서 채널을 사용할 수 있도록 Arc로 래핑

    loop {
        let (stream, _) = listener.accept().await?; // 사용자를 기다림
        let shared_tx = shared_tx.clone();
        let mut rx = shared_tx.subscribe();

        tokio::spawn(async move {
            let (reader, mut writer) = tokio::io::split(stream);
            // stream을 reader와 writer로 분리
            tokio::spawn(async move { // 이벤트 루프 가동
                loop {
                    let data: String = match rx.recv().await {
                        Ok(data) => {data},
                        Err(_) => {
                            return; // 수신 실패 시 종료
                        }
                    };

                    if data == "/quit" {
                        break;
                    }
                    
                    print!("{}", data);
                    match writer.write_all(data.as_bytes()).await {
                        Ok(_) => {},
                        Err(err) => {
                            println!("네트워크로 데이터 전송중 오류: {:?}", err);
                            return;
                        }
                    };
                }
            });

            let mut buf_reader = BufReader::new(reader);
            let mut username = String::new();

            buf_reader.read_line(&mut username).await;
            let username = username.trim();

            match shared_tx.send(format!("{} 님이 입장하셨습니다.\n", username)) {
                Ok(_) => {},
                Err(_) => {
                    return; //브로드캐스트 실패 시 종료
                }
            }

            loop {
                let mut message = String::new();
                buf_reader.read_line(&mut message).await;

                let mut message = String::from(message.trim());
                if message != "/quit" {
                    message = format!("{}: {}\n", username, message);
                }

                match shared_tx.send(message) { // 수신자에 전송
                    Ok(_) => {},
                    Err(_) => {
                        break; //브로드 캐스트 실패 시 종료
                    }
                };
            }

            match shared_tx.send(format!("{} 님이 채팅방을 나갔습니다.\n", username)) {
                Ok(_) => {},
                Err(_) => {
                    return;
                }
            }
        });
    }
}
