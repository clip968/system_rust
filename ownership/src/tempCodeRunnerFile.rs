 let mut head = Rc::new(Person {
        name:String::from("Luna"),
        age:30,
        next:RefCell::new(None),
    });

    let tail = push_back(head.clone(), String::from("Rust"), 10);
    let tail = push_back(tail.clone(), String::from("Wikibooks"), 20);

    let mut current = head.clone();
    loop {
        print!("{} -> ", current.name);
        let t = current.clone();
        // t를 복사하는 이유는 current를 그대로 갖다가 쓰면, 즉
        // current.next.borrow_mut를 해버리면 가변 참조로 current의 clone을 못하기 때문에 t로 먼저 복사해서
        // t의 값으로 match 돌리고, current를 clone 하는 방식으로 해야한다.
        current = match &(*(t.next.borrow_mut())) {
            Some(p) => p,
            None => break,
        }.clone();