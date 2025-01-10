use std::thread::{self, sleep};
use std::time::Duration;
use std::error::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use signal_hook::consts::SIGINT;
use signal_hook::iterator::Signals;
fn main() -> Result<(), Box<dyn Error>>{
    // // AtomicBool을 사용해 SIGINT 신호가 수신되었는지 여부를 확인하는 플래그를 설정
    // let signal_received = Arc::new(AtomicBool::new(false));
    // // AtomicBool에 대한 참조를 복제. 이것은 signal_hook에 전달된다.
    // let signal_received_clone = signal_received.clone();

    // // SIGINT신호를 수신할 때마다 signal_received_clone의 값을 true로 설정하는 핸들러 등록
    // signal_hook::flag::register(SIGINT, signal_received_clone)?;

    // println!("SIGINT를 수신하거나 Ctrl+C를 입력하면 종료합니다");

    // while !signal_received.load(Ordering::Relaxed) {
    //     sleep(Duration::from_secs(1));  
    // }

    // println!("SIGINT 수신");

    // Ok(())
    // 메인 루프에서 sigint 수신 여부를 지속적으로 체크하기 때문에 효율적이지 않음
    // => 별도의 스레드를 만들어 체크하는게 더 편함


    let mut signals = Signals::new(&[SIGINT])?;

    thread::spawn(move || {
        // signals.forever()를 사용하여 지속적으로 신호를 확인하고 대기 
        for sig in signals.forever() {
            println!("SIGINT 수신.");
            //프로세스 종료
            std::process::exit(0);
        }
    });

    println!("SIGINT를 수신하거나 Ctrl+C를 입력하면 종료합니다.");
    thread::sleep(Duration::from_secs(10));

    Ok(())
}
