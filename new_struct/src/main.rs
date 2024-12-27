use std::io;
#[derive(Debug)]   
struct Student {
    id : i32,
    name : String,
    email : String,
}

fn create_student(id:i32, name:String, email: String) -> Student {
    Student{
        id:id,
        name:name,
        email:email,
    }
}

struct ElementarySchool {
    room : String,
}

struct MiddleSchool {
    teacher: String,
}

struct HighSchool {
    id : i32,
}

enum SchoolKind {
    Elementary(ElementarySchool),
    Middle(MiddleSchool),
    High(HighSchool)
}

enum Message {
    Quit, 
    List(i32),
    Put(String),
    Get(i32),
}

impl Message {
    fn execute(&self) {
        match self {
            Message::Quit => println!("Quit"), // quit인 경우
            Message::List(val) => println!("List: {}", val), //List 인 경우
            Message::Put(val) => println!("Put: {}", val), // Put인 경우
            Message::Get(val) => println!("Get: {}", val), // Get인 경우
        }
    }
}


struct Score {
    score:i32,
}

impl Score {
    fn get_grade(&self) -> String {
        if self.score > 90{
            String::from("A")   
        }
        else if self.score > 80{
            String::from("B") 
        }
        else{
            String::from("C")
        }
    }
    fn from(score:i32) -> Score {
        Score {score : score}
    }
}

struct Node {
    value : i32, // 노드의 값을 저장하는 value
    next : Option<Box<Node>>, // 다음 노드를 가리키는 필드, option을 사용해 노드가 없을 수도 있는 상황을 대비(마지막 노드)
}

impl Node {
    fn append(&mut self, elem:i32){
        match self.next {
            Some(ref mut next) => {
                next.append(elem);
            }
            None => {
                let node = Node {
                    value : elem,
                    next : None,
                };
                self.next = Some(Box::new(node))
            }
        }
    }
    fn list(&self) {
        print!("{}", self.value);
        match self.next {
            Some(ref next) => next.list(),
            None => {}
        }
    }
}

fn main() {
    // println!("학번을 입력해주세요.");
    // let mut id = String::new();
    // io::stdin().read_line(&mut id);
    // let id :i32 = id.trim().parse().unwrap();

    // println!("이름을 입력해주세요.");
    // let mut name = String::new();
    // io::stdin().read_line(&mut name);
    // let name = name.trim().to_string();

    // println!("이메일을 입력해주세요.");
    // let mut email = String::new();
    // io::stdin().read_line(&mut email);
    // let email = email.trim().to_string();

    // let stu = create_student(id, name, email);
    // println!("학번 = {}, 이름 = {}, 이메일 = {}", stu.id, stu.name, stu.email); // 인스턴스의 내부값 참조
    // println!("stu={:?}", stu); debuging

    // let v = vec![1, 2, 3];
    // let t = (10, "hello", true);

    // println!("{:?}", v);  // [1, 2, 3]
    // println!("{:?}", t);  // (10, "hello", true)

    // struct Config {
    //     host: String,
    //     port: u16,
    // }
    
    // fn main() {
    //     let config = Config {
    //         host: "localhost".to_string(),
    //         port: 8080,
    //     };
    
    //     println!("{:#?}", config);

    // =>    Config {
    //         host: "localhost",
    //         port: 8080,
    //     }

    // let m = Message::Put(String::from("/root/"));
    // m.execute();

    // let m = Message::List(33);
    // m.execute();
    // Option 열거형은 값이 있을 수도 있고 없을 수도 있는 경우 사용되는 열거형이다.
    // let some_number = Some(99);
    // let some_string = Some("러스트");
    // let can_be_none : Option<i32> = None;

    // let some_string = Some(String::from("러스트"));
    // let none_string: Option<String> = None;

    // print_option(some_string);
    // print_option(none_string);

    let mut head = Node {
        value:1,
        next:None,
    };

    for i in 2..10{
        head.append(i);
    }

    head.list();

}

fn print_option(val: Option<String>){
    match val {
        Some(val ) => println!("{}", val),
        None => println!("None"),
    }
}

#[test]
fn test_get_grade(){
    // let score = Score {
    //     score:100,
    // };

    // assert_eq!(score.get_grade(), "A");

    // let score = Score {
    //     score:80,
    // };
    // assert_eq!(score.get_grade(), "C");

    assert_eq!(Score::from(100).get_grade(), "A");
    assert_eq!(Score::from(80).get_grade(), "C");
}