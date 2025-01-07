// #[derive(Debug, Clone, Copy)]
// struct Point {
//     x:i32,
//     y:i32,
// }

#[derive(Debug)]
struct Point {
    x:i32,
    y:i32,
}

fn add_points(p1: Point, p2: Point) -> Point {
    Point {
        x: p1.x + p2.x,
        y: p1.y + p2.y,
    }
}

// #[derive(Debug)]
// struct Person {
//     name: String,
//     age: u32,
//     cloned: bool,
// }

// Person을 복제. Copy와는 다르게 값을 직접 설정할 수 있음
// impl Clone for Person {
//     fn clone(&self) -> Self {
//         Person {
//             name: self.name.clone(),
//             age: self.age,
//             cloned: true,
//         }
//     }
// }

struct Book {
    title:String,
}

impl Drop for Book {
    fn drop(&mut self) {
        println!("Book 객체 해제: {}", self.title);
    }
}

impl From<(i32, i32)> for Point {
    fn from(tuple: (i32, i32)) -> Self {
        Point { x: tuple.0, y: tuple.1 }
    }
}

struct Person {
    name:String, 
    age:u32,
}

// impl AsRef<str> for Person {
//     //Person의 name을 str 형태로 참조할 수 있다.
//     fn as_ref(&self) -> &str {
//         &self.name
//     }
// }

impl AsMut<String> for Person {
    fn as_mut(&mut self) -> &mut String {
        &mut self.name
    }
}

fn name_change<P: AsMut<String>>(person:&mut P, new_name: &str) {
    let name = person.as_mut();
    name.clear();
    name.push_str(new_name);
}

fn greet_person<P: AsRef<str>>(person:P) {
    println!("안녕! {}", person.as_ref());
}

fn main() {
    // let a = Point {x:1, y:2};
    // let b = Point {x:3, y:4};

    // // add_points의 인자로 들어가는 a, b는 copy trait로 의해 복제된다.
    // // 즉 소유권을 잃지 않는다.
    // let result = add_points(a, b);

    // println!("{:?}", a);
    // println!("{:?}", b);
    // println!("{:?}", result);

    // let person1 = Person {
    //     name:String::from("루나"),
    //     age: 10,
    //     cloned:false,
    // };

    // let person2 = person1.clone();

    // println!("{:?}", person1);
    // println!("{:?}", person2);

    // {
    //     let book = Book{title:String::from("러스트")};
    // } //book의 Drop 트레잇이 자동으로 호출

    // let tuple = (1, 2);

    // let pt: Point = Point::from(tuple); // 주어진 tuple을 바탕으로 Point객체 생성

    // println!("Point::from = {:?}", pt); 

    // let pt: Point = tuple.into(); // tuple을 기반으로 point를 생성. 이때 Point::from이 호출된다
    // println!("tuple.into = {:?}", pt);

    // let person = Person {name: String::from("루나"), age:30};

    // //person 구조체에 AsRef<str>를 구현했기 때문에 
    // // greet_person 함수는 person을 인자로 받아 사용할 수 있다.
    // greet_person(person);

    // // String과 &str도 greet_person 함수 호출이 가능하다.
    // greet_person(String::from("러스트"));
    // greet_person("하이!");

    let mut person = Person {
        name: String::from("루나"),
        age: 10,
    };

    println!("변경 전 : {}", person.name);
    name_change(&mut person, "러스트");
    println!("변경 후 : {}", person.name);

}
