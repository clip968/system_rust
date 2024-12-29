use std::{cell::RefCell, rc::Rc};

type NodeType = Option<Rc<RefCell<Node>>>;

struct Node {
    item:i32, //노드에 저장된 정보값
    prev:NodeType, //이전 노드를 가리키는 포인터
    next:NodeType, //다음 노드를 가리키는 포인터
}

impl Node {
    fn new(item: i32) -> Self {
        Self {
            item,
            prev:None,
            next:None,
        }
    }
}

pub struct DoubleLinkedList {
    head:NodeType,
    tail:NodeType,
}

impl DoubleLinkedList {
    fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    fn push_back(&mut self, item: i32) {
        // 새로운 노드를 생성
        let node = Rc::new(RefCell::new(Node::new(item)));

        // tail이 있는지 확인. 만약 tail이 있다면 새로운 노드를 리스트의 끝에 삽입
        if let Some(tail) = self.tail.take() {
            tail.borrow_mut().next = Some(Rc::clone(&node)); // 현재 tail의 next 노드를 새로 만든 노드로 설정
            node.borrow_mut().prev = Some(tail); // 새 노드의 prev를 지금의 tail로 설정
            self.tail = Some(node); // 새 노드를 tail로 설정
        }
        else {
            // tail이 없다면 
            self.head = Some(Rc::clone(&node));
            self.tail = Some(node);
        }
    }

    fn push_front(&mut self, item: i32) {
        let node = Rc::new(RefCell::new(Node::new(item)));

        if let Some(head) = self.head.take(){
            head.borrow_mut().prev = Some(Rc::clone(&node));
            node.borrow_mut().next = Some(head);
            self.head = Some(node);
        }
        else {
            self.head = Some(Rc::clone(&node));
            self.tail = Some(node);
        }
    }

    fn print_all(&mut self) {
        let mut current = match &self.head {
            Some(n) => n.clone(),
            None => return,
        };

        loop {
            let t = current.clone();
            println!("item: {}", t.borrow().item);
            current = match &t.borrow().next{
                Some(n) => {
                    n
                },
                None => break,
            }.clone();
        }
    }
}

fn main() {
    let mut list = DoubleLinkedList::new();

    println!("뒤에 1, 2, 3 삽입");
    list.push_back(1);
    list.push_back(2);  
    list.push_back(3);

    list.print_all();

    println!("맨 앞에 0 추가");
    list.push_front(0);
    list.print_all();
}
