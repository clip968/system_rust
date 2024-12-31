use std::collections::{BinaryHeap, HashMap, HashSet, LinkedList};
// extern crate encoding_rs;
// use encoding_rs::{EUC_KR, UTF_8};
// use std::str;

fn main() {
    // let ret = div(1, 0);
    // println!("ret : {}", ret);

    //Vec은 보통 벡터라고 부른다.
    // 용량 제한이 없고 유동적임.
    // let mut v : Vec<i32> = Vec::new();
    // let mut v : Vec<i32> = vec![1,2,3];

    // for i in 1..10 {
    //     v.push(i);  
    // }

    // for d in &v {
    //     print!("{}, ", d);
    // }

    // let v = vec![1,2,3];

    // let one = v[0];
    // let two = v.get(1);
    // .get은 Option 타입을 반환하므로 Some(2)를 반환하게 된다.
    // 즉 해당 인덱스에 값이 없으면 None을 반환한다.
    // get 대신에 만약 v[9] 이런식으로 배열의 크기를 넘어버리면 이때는 None이 아니라 panic!이 실행된다

    // println!("One : {:?}, Two : {:?}", one, two);

    // let mut v = vec![1,2,3];
    // v[0] = 2;
    // v[1] = 3;
    // v[2] = 4;

    // println!("{} {} {}", v[0], v[1], v[2]);
    //2 3 4

    // let mut v = vec![1,2,3];
    // for i in &mut v {
    //     *i += 1;
    // }

    // println!("{} {} {}", v[0], v[1], v[2]);
    // 2 3 4

    // let mut list : LinkedList<i32> = LinkedList::new();
    // list.push_back(1);
    // list.push_back(2);
    // list.push_back(3);

    // for i in &list {
    //     print!("{}, ", i);
    // }

    // let mut list: LinkedList<i32> = LinkedList::new();
    // for i in 0..10 {
    //     list.push_back(i);
    // }

    // let idx = 9;
    // let mut i = 0;
    // let mut target = 0;

    // for data in &list {
    //     if i == idx {
    //         target = *data;
    //     }
    //     i += 1;
    // }

    // println!("target: {:?}", target);

    // list.iter().nth(9); 9번째 인덱스에 있는 값을 가져옴.
    // Option이라 Some이 나온다.

    // let mut list: LinkedList<i32> = LinkedList::new();
    // for i in 0..10 {
    //     list.push_back(i);
    // }

    // linkedlist 내부 값을 mut로 수정할 수 있음
    // for d in list.iter_mut() {
    //     *d += 10;
    // }

    // for d in list.iter() {
    //     print!("{:?}, ", d);
    // }

    // Hashmap은 키와 값으로 이루어진 자료를 관맇나다.
    // 보통 캐시를 구현할 때 많이 사용한다.
    // let mut books: HashMap<i32, String> = HashMap::new();

    // books.insert(10, String::from("Rust"));
    // books.insert(20, String::from("Java"));
    // books.insert(30, String::from("Python"));

    // for (key, value) in &books {
    //     println!("Key : {:?}, Value : {:?}", key, value);
    // }

    // let rust = books.get(&10).unwrap();
    // println!("key 10은 {:?}", rust);

    //hashset은 집합을 의미한다.
    //집합 내 모든 값은 중복되지 않고 고유의 값을 유지한다. 보통 중복 제거용으로 많이 사용
    // let mut book: HashSet<String> = HashSet::new();

    // book.insert(String::from("Rust"));
    // book.insert(String::from("Rust"));
    // book.insert(String::from("Rust"));
    // book.insert(String::from("Java"));

    // for data in &book {
    //     println!("{}", data);
    // }

    // contains을 사용하면 값이 있는지 없는지 확인할 수 있다.
    // if book.contains("Python") == false {
    //     println!("파이썬이 없습니다");
    // }


    // Binaryheap은 다른말로 우선순위 큐라고도 부른다
    // 즉 가장 큰 값, 혹은 작은 값을 맨 위에 올려둔다.
    // let mut heap: BinaryHeap<i32> = BinaryHeap::new();

    // heap.push(3);   
    // heap.push(9);
    // heap.push(2);
    // heap.push(5);

    // 숫자 다 넣고 계속 pop 시키면 사실상 정렬하는 거랑 똑같은 결과를 가짐
    // while heap.is_empty() == false {
    //     print!("{:?}, ", heap.pop());
    //     // pop은 Option값이라 Some을 반환함.
    // }

    // String과 str은 좀 다르다.
    // &str은 배열로 문자열을 관리하고 STring은 동적으로 관리하고 소유권을 가지고 있다.

    // let mut eng = String::new();
    // eng.push_str("Hello");
    // let kor = "안녕하세요".to_string();

    // println!("{} {}", eng, kor);

    // let str = String::from("안녕");
    // let idx = 123;

    // // 두 숫자와 문자열을 합치는 함수
    // let s = format!("{}{}", str, idx);
    // println!("{}", s);

    // let txt = String::from("안녕하세요");
    // for c in txt.chars() { // txt안의 문자열을 순회하는 과정 
    //     print!("{}, ", c);
    // }

    // let utf8_string = "안녕하세요";
    
    // // utf-8로 인코딩된 바이트로 변환
    // let utf8_bytes = utf8_string.as_bytes();

    // // euc-kr로 인코딩
    // let (euc_kr_bytes, _, _) = EUC_KR.encode(utf8_string);

    // println!("utf-8: {:?}", utf8_bytes);
    // println!("euc-kr: {:?}", euc_kr_bytes);

    // let (utf8_string, _, _) = EUC_KR.decode(&euc_kr_bytes);

    // println!("euc-kr to utf-8: {}", utf8_string);

    // let vec = vec![1,2,3];
    // for item in vec.iter() { // vec의 불변 빌림 반복자 생성
    //     println!("{}", item);
    // }

    // println!("{:?}", vec); //접근 가능

    // let vec = vec![1,2,3];
    // for item in vec.into_iter() {
    //     // vec의 소유권이 이동하여 다음에 vec를 다시 사용할 수 없음
    //     println!("{}", item);
    // }

    // println!("{:?}", vec); // 사용 불가





}

// fn div(a: i32, b:i32) -> i32 {
//     a / b
// }