use std::io;
fn main() {
    println!("n번째 수를 입력해주세요.");

    let mut n = String::new();

    io::stdin().read_line(&mut n);
    let n : i32 = n.trim().parse().unwrap();
    
    let ret = fibo(n);
    println!("결과 : {}", ret);
}

fn fibo(n:i32) -> i32{
    let mut next = 0; 
    let mut t1 = 1;
    let mut t2 = 1;
    let mut counter = 2;
    print!("1, 1, ");
    while counter < n{
        next = t1 + t2;
        t1 = t2;
        t2 = next;
        print!("{}, ", next);

        counter += 1;
    }
    println!("");

    next

}

#[test]
fn fibo_test(){
    assert_eq!(fibo(6), 8);
    assert_eq!(fibo(7), 13);
}
