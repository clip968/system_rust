use hyper::body::{self, Buf};
use hyper::Client;
use serde::Deserialize;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, GenericError>;

#[derive(Deserialize, Debug)]
struct User {
    id: i32,
    name:String,
}

async fn response_examples(req: Request<Body>) -> Result<Response<Body>> {
    let index_html = String::from("<h1>Hello World</h>");
    let notfound_html = String::from("<h1>404 not found</h>");

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => Ok(Response::new(index_html.into())),
        _ => {
            //404 오류 발생
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(notfound_html.into())
                .unwrap())
        }
    }
}

#[tokio::main]
async fn main() -> Result<()>{
    // let client = Client::new();
    // let uri = "http://httpbin.org/ip".parse().unwrap();
    // let mut resp = client.get(uri).await.unwrap();
    // println!("Response: {}", resp.status());

    // while let Some(chunk) = resp.body_mut().data().await {
    //     stdout().write_all(&chunk.unwrap()).await.unwrap();
    // }

    // let url = "http://jsonplaceholder.typicode.com/users".parse().unwrap();
    // let client = Client::new(); // 새로운 http 클라이언트를 실행한다.
    // let res = client.get(url).await.unwrap(); // http get 요청을 url로 보낸다

    // let body = hyper::body::aggregate(res).await.unwrap(); // res의 본문(body)를 읽고 하나의 body로 aggregate(합친다)
    // 결과적으로 body는 http 응답 본문을 포함하고 있는 데이터 덩어리이다.

    // let users: Vec<User> = serde_json::from_reader(body.reader()).unwrap();
    // 받은 Json을 serde로 역직렬화

    // println!("사용자: {:#?}", users);

    let addr = "127.0.0.1:8080".parse().unwrap();
    let new_service = make_service_fn(move |_| {
        async {
            Ok::<_, GenericError>(service_fn(move |req| {
                response_examples(req)
            }))
        }
    });

    let server = Server::bind(&addr).serve(new_service);
    println!("Listening on http://{}", addr);
    server.await?;
    Ok(())
}
