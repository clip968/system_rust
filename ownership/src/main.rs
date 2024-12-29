use std::rc::{self, Rc, Weak};
use std::cell::RefCell;
#[derive(Copy, Clone)]
struct Student {
    age : i32,
}

// struct Person {
//     name : String,
//     age :i32,
//     next : Option<Rc<Person>>,
// }

// struct Person{
//     name:String,
//     age:i32,
//     next:RefCell<Option<Rc<Person>>>,
// }

struct Person {
    id:i32,
    next:RefCell<Option<Weak<Person>>>,
}

impl Drop for Person {
    fn drop(&mut self) {
        println!("p{} Drop!", self.id);
    }
}



fn main() {
    // 소유권은 러스트가 가진 가장 큰 특징
    // let s1 = String::from("Hello Rust");

    // let s2 = s1;

    // println!("{}", s1); // 소유권이 s2로 넘어갔기 때문에 컴파일 에러
    // println!("{}", s2);

    // -------------------------------------

    // let s1 = String::from("Hello Rust");

    // let s2 = s1.clone();

    // println!("{}", s1); // s1의 소유권은 그대로 두고 s1의 값만을 s2에 복재했기 때문에 소유권 자체는 그대로 있음

    // -------------------------------------

    // let s = String::from("Hello");
    // push_str(s); // push_str 함수에 s의 소유권 이관
    // println!("{}",s); // s의 소유권이 push_str로 이관되어 컴파일 X

    // let s = String::from("Hello");
    // let s = push_str(s); //섀도잉 방식으로 push_str로부터 다시 소유권을 가져옴
    // println!("{}", s);
    
    // let mut s = String::from("Hello");
    // push_str(&mut s); // &을 사용하여 소유권을 이전하지 않고 참조를 전달하여 값 변경
    // println!("{}", s);

    // let mut s1 = Student{age : 10};
    // let s2 = s1; // s1을 복사하여 s2에 저장한다 (copy trarit)

    // println!("s1 : {}, s2 : {}", s1.age, s2.age);

    // s1.age = 20;

    // println!("s1 : {}, s2 : {}", s1.age, s2.age);
    // 다만 Copy trait는 string은 못함. 이 경우 s2 = s1.clone()으로 해결해야함

    // let mut x = Box::new(10); 
    // // Box를 이용한 동적 할당 -> 힙에 할당하는 방식
    // println!("x = {}", x);
    // *x = 20;
    // // 할당 받은 메모리에 접근하려면 *를 사용해야 한다.
    // println!("x = {}", x);
    // // 동적으로 생성한 X를 따로 해제해 주지 않아도 데이터가 사용되지 않는 시점에서 자동으로 해제된다.

    // Rc는 레퍼런스 카운팅 포인터이다
    // Rc로 관리되는 데이터는 공유가 가능해 여러 변수가 동일한 값을 참조할 수 있도록 한다.
    // 이 때 RC는 데이터를 참조하는 모든 변수들이 메모리에 존재하는 동안 해당 값을 해제하지 않는다.
    // let person = Rc::new(Person{age:10});

    // let p1 = person.clone();
    // println!("person:{} p1: {}", person.age, p1.age);
    // println!("RefCount : {}", Rc::strong_count(&person));

    // let p2 = person.clone();
    // println!("RefCount : {}", Rc::strong_count(&person));
    // {
    //     let p3 = person.clone();
    //     println!("RefCount : {}", Rc::strong_count(&person));
    // }
    // // p3 해제

    // println!("RefCount : {}", Rc::strong_count(&person));

    // let p1 = Rc::new(Person {
    //     name: String::from("Luna"),
    //     age : 30,
    //     next:None,
    // });

    // let p2 = Rc::new(Person{
    //     name: String::from("Rust"),
    //     age: 28,
    //     next: Some(p1.clone()),
    // });

    // print!("{} -> ", p2.name);

    // match &p2.next {
    //     Some(p) => {
    //         println!("{}", p.name);
    //     },
    //     None => {},
    // };

    // let head = Rc::new(Person {
    //     name: String::from("Luna"),
    //     age:30,
    //     next: None,
    // });

    // let head = push_front(head, String::from("Rust"), 10);
    // let head = push_front(head, String::from("Wikibooks"), 20);

    // let mut current = head.clone();

    // loop {
    //     print!("{} -> ", current.name);
    //     current = match &current.next {
    //         Some(p) => p,
    //         None => break,
    //     }.clone();
    // }

    // Rc는 범용성이 좋지만 불변성을 가진 참조형이기 때문에 공유 데이터를 변경할 수 없다.
    // let mut p1 = Rc::new(Person {
    //     name:String::from("Luna"),
    //     age:30,
    //     next:RefCell::new(None),
    // });

    // let mut p2 = Rc::new(Person {
    //     name:String::from("Rust"),
    //     age:10,
    //     next:RefCell::new(None),
    // });

    // let mut next = p1.next.borrow_mut();
    // *next = Some(p2.clone());

    // let mut head = Rc::new(Person {
    //     name:String::from("Luna"),
    //     age:30,
    //     next:RefCell::new(None),
    // });

    // let tail = push_back(head.clone(), String::from("Rust"), 10);
    // let tail = push_back(tail.clone(), String::from("Wikibooks"), 20);

    // let mut current = head.clone();
    // loop {
    //     print!("{} -> ", current.name);
    //     let t = current.clone();
    //     // t를 복사하는 이유는 current를 그대로 갖다가 쓰면, 즉
    //     // current.next.borrow_mut를 해버리면 가변 참조로 current의 clone을 못하기 때문에 t로 먼저 복사해서
    //     // t의 값으로 match 돌리고, current를 clone 하는 방식으로 해야한다.
    //     current = match &(*(t.next.borrow_mut())) {
    //         Some(p) => p,
    //         None => break,
    //     }.clone();
    //     // 가변 참조는 해당 코드 내에 어떤 가변참조, 불변참조조 있어선 안된다.
    //     // 즉 가변참조 할거면 해당 데이터에 누구도 접근하고 있으면 안된다
    // }
    
    let mut p1 = Rc::new(Person {
        id:1,
        next:RefCell::new(None),
    });

    let mut p2 = Rc::new(Person {
        id:2,
        next:RefCell::new(None),
    });

    let mut next = p1.next.borrow_mut();
    *next = Some(Rc::downgrade(&p2));

    let mut next = p2.next.borrow_mut();
    *next = Some(Rc::downgrade(&p1));

    println!("p1 RefCount : {}, p2 : RefCount : {}", Rc::strong_count(&p1), Rc::strong_count(&p2));

}

// fn push_str(mut s:String) -> String {
//     s.push_str("Rust!"); 
//     s // 섀도잉으로 다시 소유권 돌려주기
// }

// fn push_str(s: &mut String){
//     s.push_str(" Rust!");
// }

// fn push_front(head: Rc<Person>, name: String, age:i32) -> Rc<Person> {
//     //새로운 person 노드 생성
//     let p = Rc::new(Person {
//         name: name,
//         age: age,
//         next: Some(head.clone()),
//     });

//     p.clone()
// }

// fn push_back(tail: Rc<Person>, name:String, age:i32) -> Rc<Person> {
//     let p = Rc::new(Person {
//         name:name,
//         age: age,
//         next:RefCell::new(None),
//     });

//     let mut next = tail.next.borrow_mut();
//     *next = Some(p.clone());

//     p
// }

