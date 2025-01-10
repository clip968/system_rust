use dbus::blocking::Connection;
use dbus_crossroads::{Crossroads, Context};
use std::error::Error;

struct Hello {
    called_count : u32
}

fn main() -> Result<(), Box<dyn Error>>{
    //새로운 Dbus 세션 연결을 생성
    let c = Connection::new_session()?;
    c.request_name("com.example.dbustest", false, true, false)?;

    //Crossrads 인스턴스 생성
    let mut cr = Crossroads::new();

    let iface_token = cr.register("com.example.dbustest", |b| {
        //HelloHappen이라는 dbus signal을 설정
        let hello_happened = b.signal::<(String,), _>("HelloHappened", ("sender",)).msg_fn();

        // hello 메서드를 정의한다
        b.method("Hello", ("name",), ("reply",), move |ctx: &mut Context, hello: &mut Hello, (name,): (String,)| {
            println!("클라이언트 쿼리: 안녕 {}!", name);
            hello.called_count += 1;
            let reply = format!("안녕 {}! API 호출 횟수: {}", name, hello.called_count);
            let signal_msg = hello_happened(ctx.path(), &(name,));
            ctx.push_msg(signal_msg);
            Ok((reply,))
        });
        
    });

    // hello라는 경로에 hello 구조체 인스턴스를 삽입하고 인터페이스 토큰을 연결
    cr.insert("/hello", &[iface_token], Hello { called_count: 0 });

    // 생성된 crossroads 인스턴스를 사용해 DBus 연결에서 메세지를 처리하도록 서비스 시작

    cr.serve(&c)?;

    Ok(())
}
