use autocxx::prelude::*;

include_cpp! {
    #include "input.h"
    // FFI 호출이 안전하지 않음을 명시
    safety!(unsafe_ffi)
    // input.h의 x3함수를 러스트에 노출
    generate!("x3")
    // input.h의 test 클래스를 러스트에 노출
    generate!("Test")
}

fn main() {
    // x3 함수를 호출
    println!("4 x 3 = {}", ffi::x3(4));

    let mut test = ffi::Test::new().within_box();
    
    test.as_mut().inc();
    test.as_mut().inc();

    println!("{}", test.to_string().as_ref().unwrap().to_string_lossy());
}
