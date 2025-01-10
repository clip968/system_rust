use dbus::blocking::Connection;
use std::time::Duration;
fn main() -> Result<(), Box<dyn std::error::Error>>{
    //새로운 DBUs 세션 연결
    let conn = Connection::new_session()?;

    //DBus 프락시 생성
    //com.example.dbustest 서비스의 hello 경로에 접근
    let proxy = conn.with_proxy("com.example.dbustest", "/hello", Duration::from_millis(5000));

    // hello 호출
    let (hello,): (String,) = proxy.method_call("com.example.dbustest", "Hello", ("luna",))?;

    println!("수신: {}", hello);

    Ok(())

}
