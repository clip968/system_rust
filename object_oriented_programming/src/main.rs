// pub struct Student {
//     id: i32, //private 필드
//     pub name: String, // public 필드
//     pub email: String, // public 필드
// }

// impl Student {
//     //public 생성자
//     pub fn new(id: i32, name: String, email: String) -> Student {
//         Student{id, name, email}    
//     }

//     pub fn get_name(&self) -> &String {
//         &self.name
//     }

//     fn set_name(&mut self, name:String) {
//         self.name = name.clone();
//     }
// }

use std::{collections::HashMap, rc::Rc};

trait Hello {
    fn hello_msg(&self) -> String;
}

fn say_hello(say: &dyn Hello) {
    println!("{}", say.hello_msg());
}

struct Student{}

impl Hello for Student {
    fn hello_msg(&self) -> String {
        String::from("안녕하세요 선생님, ")
    }
}

struct Teacher {}

impl Hello for Teacher {
    fn hello_msg(&self) -> String {
        String::from("안녕하세요 오늘 수업은, ")
    }
}

trait Pointable {
    fn x(&self) -> i32;
    fn y(&self) -> i32;
}

struct Point {
    x: i32,
    y: i32,
}

impl Pointable for Point {
    fn x(&self) -> i32 {
        self.x
    }
    fn y(&self) -> i32 {
        self.y
    }
}

fn print_pointable(pointable : &dyn Pointable) {
    println!("x: {}, y: {}", pointable.x(), pointable.y());
}

struct ColorPoint {
    color: String,
    point: Point,
}

impl ColorPoint {
    fn new(color: String, x:i32, y:i32) -> ColorPoint {
        ColorPoint {
            color: color,
            point: Point { x: x, y: y }
        }
    }

    fn color(&self) -> &String {
        &self.color
    }
}

impl Pointable for ColorPoint {
    fn x(&self) -> i32 {
        self.point.x
    }
    fn y(&self) -> i32 {
        self.point.y
    }
}

// factory method 패턴
trait Pizza {
    fn eat(&self);
}

enum PizzaType {
    Bulgogi,
    Hawaiian,
}
// 피자의 종류에 따라 적절한 피자 객체를 생성하는 메서드
trait PizzaFactory {
    fn create(pizza_type: PizzaType) -> Box<dyn Pizza>; // 주어진 피자 종류에 따라 해당하는 피자 객체 반환
}

struct ConcretePizzaFactory {}
struct BulgogiPizza {}
struct Hawaiianpizza {}

impl PizzaFactory for ConcretePizzaFactory {
    fn create(pizza_type: PizzaType) -> Box<dyn Pizza> {
        match pizza_type {
            PizzaType::Bulgogi => Box::new(BulgogiPizza{}), 
            PizzaType::Hawaiian => Box::new(Hawaiianpizza{}),
        }
    }
}

impl Pizza for BulgogiPizza {
    fn eat(&self) {
        println!("불고기 냠냠");
    }
}

impl Pizza for Hawaiianpizza {
    fn eat(&self) {
        println!("파인애플 냠냠")
    }
}

// SingleTon 패턴
#[macro_use]
// lazy static을 통해 mysingleton인스턴스를 한 번만 생성하도록 한다.
// 이렇게 하면 프로그램 전체에서 하나의 인스턴스만 사용하게 되어 싱글턴 패턴이 구현된다.
extern crate lazy_static;
lazy_static!{
    static ref INSTANCE: MySingleton = {
        MySingleton::new(String::from("luna"))
    };
}

struct MySingleton {
    name:String,
}

impl MySingleton {
    fn new(name:String) -> MySingleton {
        MySingleton{name}
    }
    
    fn call(&self) {
        println!("my name is {}", self.name);
    }
}
 // builder 패턴
struct Burger {
    bun: String,
    patties: i32,
    sauce: String,
    extra: Vec<String>,
}

impl Burger {
    fn to_string(&self) -> String {
        let mut txt = format!("{} 위에 순 쇠고기 패티 {}장 {} 소스 ", self.bun, self.patties, self.sauce);

        for ex in self.extra.iter(){
            txt = format!("{} {}", txt, ex);
        }

        txt
    }
}

struct BurgerBuilder {
    bun: String,
    patties: i32,
    sauce: String,
    extra: Vec<String>,
}

impl BurgerBuilder {
    fn new() -> BurgerBuilder {
        BurgerBuilder { bun: String::from(""), patties: 0, 
        sauce: String::from(""), extra: Vec::<String>::new() }
    }

    fn bun(mut self, bun: String) -> BurgerBuilder {
        self.bun = bun;
        self
    }

    fn patties(mut self, patties:i32) -> BurgerBuilder {
        self.patties = patties;
        self
    }

    fn sauce(mut self, sauce:String) -> BurgerBuilder {
        self.sauce = sauce;
        self
    }

    fn add_extra(mut self, val: String) -> BurgerBuilder {
        self.extra.push(val);
        self
    }
    
    fn build(self) -> Burger {
        Burger { bun: self.bun, patties: self.patties, sauce: self.sauce, extra: self.extra }
    }
}

// 어댑터 패턴
// 기존에 사용하던 시스템의 인터페이스와 호환되지 않는 상황에서 인터페이스를 동일하게 유지하고 싶을때 사용
struct Adaptee {}

impl Adaptee {
    fn new() -> Adaptee {
        Adaptee {}  
    }

    fn vendor_specific_api(&self) {
        println!("벤더가 정의한 API");
    }
}

//adapter 구조체는 클라이언트가 사용하려는 인터페이스를 제공
//이 구조체는 adaptee에 대한 연결을 캡슐화 하며 클라이언트와 호환되는 인터페이스를 제공

struct Adapter {}

impl Adapter {
    fn new() -> Adapter {
        Adapter{}
    }

    // 우리가 정의한 api
    // 클라이언트가 호출할 때 이 메서드는 내부적으로 adaptee의 벤더 지정 api를 호출
    // 따라서 클라이언트와 adaptee 간의 인터페이스 차이 극복
    fn call_api(&self) {
        Adaptee::new().vendor_specific_api();
    }
}

// composite 패턴
trait Control {
    fn draw(&self) -> String;
}

struct Button {
    name: String
}

impl Control for Button {
    fn draw(&self) -> String {
        format!("Button - {}", self.name)
    }
}

impl Button {
    fn new(name:String) -> Box<Button> {
        Box::new(Button {
            name: name,
        })
    }
}

struct Panel {
    name: String,
    controls: Vec<Box<dyn Control>>,
}

impl Control for Panel {
    fn draw(&self) -> String {
        let mut txt = format!("Panel - {}", self.name);
        for control in self.controls.iter() {
            txt = format!("{}\n\t{}", txt, control.draw());
        }
        txt
    }
}

impl Panel {
    fn new(name: String) -> Box<Panel> {
        Box::new(Panel { name: name, controls: Vec::new() })
    }

    fn add_control(&mut self, control: Box<dyn Control>) {
        self.controls.push(control);
    }
}

// decorator 패턴

trait Control_1 {
    fn draw(&self) -> String;
}

struct Button_1 {
    name: String,
    decorators : Vec<Box<dyn Control_1>>
}

impl Control_1 for Button_1 {
    fn draw(&self) -> String {
        let mut txt = format!("{}", self.name);

        for decorator in self.decorators.iter() {
            txt = format!("{} and {}", txt, decorator.draw());
        }

        txt
    }
}

impl Button_1 {
    fn new(name:String) -> Button_1 {
        Button_1 {
            name: name,
            decorators: Vec::new(),
        }
    }

    fn add_decorator(&mut self, decorator: Box<dyn Control_1>) {
        self.decorators.push(decorator);
    }
}

struct Deco {
    name: String,
}

impl Control_1 for Deco {
    fn draw(&self) -> String {
        format!("{}", self.name)
    }
}

impl Deco {
    fn new(name: String) -> Box<Deco> {
        Box::new(Deco {
            name: name
        })
    }
}

// flyweight 패턴
trait Flyweight {
    fn get_name(&self) -> String;
}

struct ConcreteFlyweight {
    name: String,
}

impl Flyweight for ConcreteFlyweight {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

struct FlyweightFactory {
    flyweights: HashMap<String, Rc<Box<dyn Flyweight>>>
}

impl FlyweightFactory {
    fn new() -> FlyweightFactory {
        FlyweightFactory {
            flyweights: HashMap::new()
        }
    }

    fn get_flyweight(&mut self, name:String) -> Rc<Box<dyn Flyweight>> {
        if let Some(instance) = self.flyweights.get(&name) {    
            return instance.clone();            
        }

        let instance = Box::new(ConcreteFlyweight {
            name: name.clone(),
        });

        let instance = Rc::new(instance as Box<dyn Flyweight>);
        self.flyweights.insert(name.clone(), instance.clone());

        instance.clone()
    }
}
// observer 패턴
#[derive(PartialEq)]
struct Listener {}

impl Listener {
    fn on_event(&self, data: &str) {
        println!("Event 발생: {}", data);
    }
}

struct Subject {
    observers: Vec<Rc<Listener>>
}

impl Subject {
    fn subscribe(&mut self, observer: Rc<Listener>) {
        self.observers.push(observer);
    }

    fn unsubscribe(&mut self, observer: Rc<Listener>) {
        if let Some(index) = self.observers.iter().position(|o| *o == observer) {
            self.observers.remove(index);
        }
    }

    fn notify(&self, data: &str) {
        for observer in &self.observers {
            observer.on_event(data);
        }
    }
}

// strategy 패턴

trait Render {
    fn render(&self, title: String, body: String);
}

struct HtmlRenderer {}

impl Render for HtmlRenderer {
    fn render(&self, title: String, body: String) {
        println!("<html><title>{}</title><body>{}</body></html>", title, body);
    }
}

struct MarkdownRenderer;

impl Render for MarkdownRenderer {
    fn render(&self, title: String, body: String) {
        println!("# {}\n{}", title, body);
    }
}

struct Page<T: Render> {
    renderer: T,
}

impl<T:Render> Page<T> {
    fn new(renderer: T) -> Page<T> {
        Page { renderer }
    }

    fn render(&self, title: String, body: String) {
        self.renderer.render(title, body);
    }
}
// state 패턴
trait State {
    fn on_state_changed(self: Box<Self>) -> Box<dyn State>;
}

struct Start;
impl State for Start {
    fn on_state_changed(self: Box<Start>) -> Box<dyn State> {
        println!("현재 상태는 [start]. 다음 상태는 [Running]");
        Box::new(Running {})
    }
}

struct Running;
impl State for Running {
    fn on_state_changed(self: Box<Running>) -> Box<dyn State> {
        println!("현재 상태는 [Running], 다음 상태는 [Stop]");
        Box::new(Stop {})   
    }
}

struct Stop;
impl State for Stop {
    fn on_state_changed(self: Box<Stop>) -> Box<dyn State> {
        println!("현재 상태는 [Stop]. 다음 상태는 [없음]");
        self
    }
}

struct Boot {
    state: Option<Box<dyn State>>
}

impl Boot {
    fn new() -> Boot {
        Boot {
            state: Some(Box::new(Stop {}))
        }
    }

    fn boot(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(Box::new(Start {}))
        }
    }

    fn next(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.on_state_changed())
        }
    }
}

fn main() {
    // let student = Student::new(1, String::from("luna"), String::from("luna@email.me"));
    // println!("이름: {}", student.get_name());
    // let student = Student {};
    // let teacher = Teacher {};

    // say_hello(&student);
    // say_hello(&teacher);

    // let pt = ColorPoint::new(String::from("red"), 1, 2);
    // print_pointable(&pt);

    // let bulgogi = ConcretePizzaFactory::create(PizzaType::Bulgogi);
    // let hawaiian = ConcretePizzaFactory::create(PizzaType::Hawaiian);

    // bulgogi.eat();
    // hawaiian.eat();

    // INSTANCE.call();

    // let burger = BurgerBuilder::new()
    //     .bun(String::from("참깨빵"))
    //     .patties(2)
    //     .sauce(String::from("특별한"))
    //     .add_extra(String::from("양상추"))
    //     .build();

    // println!("{}", burger.to_string());

    // let adapter = Adapter::new();
    // adapter.call_api();

    // let mut root = Panel::new(String::from("root"));
    // root.add_control(Button::new(String::from("button #1")));

    // let mut panel = Panel::new(String::from("panel #1"));
    // panel.add_control(Button::new(String::from("button #2")));
    // root.add_control(panel);

    // println!("{}", root.draw());

    // let mut button = Button_1::new(String::from("참깨빵"));
    // button.add_decorator(Deco::new(String::from("순쇠고기")));
    // button.add_decorator(Deco::new(String::from("패티두장")));

    // println!("{}", button.draw());

    // let mut factory = FlyweightFactory::new();

    // let flyweight1 = factory.get_flyweight(String::from("위키"));
    // let flyweight2 = factory.get_flyweight(String::from("북스"));
    // let flyweight3 = factory.get_flyweight(String::from("북스"));

    // println!("{}", flyweight1.get_name());
    // println!("{}", flyweight2.get_name());
    // println!("{}", flyweight3.get_name());

    // let mut subject = Subject {
    //     observers: Vec::new()   
    // };

    // let observer1 = Rc::new(Listener {});
    // let observer2 = Rc::new(Listener {});

    // subject.subscribe(observer1.clone());
    // subject.subscribe(observer2.clone());

    // subject.notify("이벤트 #1");

    // subject.unsubscribe(observer1.clone());

    // subject.notify("이벤트 #2"); 

    // let html = Page::new(HtmlRenderer {});
    // html.render(String::from("제목"), String::from("내용"));

    // let markdown = Page::new(MarkdownRenderer {});
    // markdown.render(String::from("제목"), String::from("내용"));

    let mut post = Boot::new();
    post.boot();
    post.next();
    post.next();
    post.next();

}
