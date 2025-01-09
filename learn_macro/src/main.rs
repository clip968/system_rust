// expr : 표현식
// ident : 식별자
// tt : 토큰트리
// ty : 타입
// pat : 패턴

macro_rules! say_hello {
    // 매크로의 입력으로 사용되는 표현식을 받아와 $name에 바인딩
    // $기호는 매크로 인자를 나타내며 이는 매크로의 입력으로 사용됨
    // 즉 $name:expr는 매크로가 하나의 인자를 받아야하며 그 인자는 표현식이라는 것을 나타냄
    ($name:expr) => {
        println!("안녕! {}", $name);
    };
}

macro_rules! add {
    ($a:expr, $b:expr) => {
        $a + $b 
    };
}

macro_rules! create_function { // 함수 생성 매크로
    ($func_name:ident) => { // 함수 이름을 받아 func_name에 저장한다
        fn $func_name() { // func_name으로 함수를 생성
            println!("함수 : {:?}()", stringify!($func_name));
        }
    };
}

create_function!(ident_func);

macro_rules! create_accessors {
    ($name:ident, $type:ty, $setter:ident) => {
        fn $name(&self) -> &$type {
            &self.$name
        }

        fn $setter(&mut self, value: $type) {
            self.$name = value;
        }
    };
}

struct Person {
    name: String,
    age: u32,
}

impl Person {
    create_accessors!(name, String, set_name);
    create_accessors!(age, u32, set_age);
}

macro_rules! multi_var {
    ($($var:ident: $type:ty),*) => { // 변수 명과 타입을 여러 개 입력받는다.
        $(
            let mut $var: $type = Default::default();
        )*
    };
}

macro_rules! multiply {
    ($x: expr, $y: expr) => {
        $x * $y
    };
}

macro_rules! compute {
    ($a:expr, $b:expr, $c: expr, $d:expr) => {
        multiply!(add!($a, $b), add!($c, $d))
    };
}

macro_rules! S {
    ($e:expr) => {
        String::from($e)
    };
}

fn main() {
    // say_hello!("러스트");
    // let sum = add!(1, 2);
    // println!("1 + 2 = {}", sum);

    // ident_func();

    // let mut person = Person { name: "루나".to_string(), age: 10 };
    // person.set_name("하이".to_string());
    // person.set_age(8);  

    // println!("이름: {}, 나이: {}", person.name, person.age);

    // multi_var!(x:u32, y:f64, z:String);
    // multi_var!의 결과값 => 
    // let mut x: u32 = Default::default();
    // let mut y: f64 = Default::default();
    // let mut z: String = Default::default();

    // let result = compute!(1,2,1,2);
    // println!("(1+2)x(1+2) = {}", result);

    let world = S!("World");
    println!("Hello, {}", world);

}
