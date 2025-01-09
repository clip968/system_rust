use std::mem;
fn main() {
    // let src = [1, 2, 3];
    // let mut dest = [0; 3];

    //안전하지 않은 작업을 수행하기 위해 unsafe 블록 내에서 호출됩니다.
    //copy_nonoverlapping 함수는 src의 포인터에서 dest의 포인터로 메모리를 복사한다.
    // 이때, 복사되는 길이(src.len())는 반드시 dest의 크기보다 작거나 같아야 한다.

    // unsafe {
    //     std::ptr::copy_nonoverlapping(src.as_ptr(), dest.as_mut_ptr(), src.len());
    //     // len의 크기는 반드시 dest보다 작거나 같아야함
    // }

    // println!("{:?}", dest);

    // unsafe {
    //     // libc::malloc 함수를 사용해 i32 크기 만큼의 힙 메모리를 할당하고 할당된 메모리의 포인터를 ptr에 저장
    //     let ptr: *mut i32 = libc::malloc(mem::size_of::<i32>()) as *mut i32;

    //     if ptr.is_null() {
    //         panic!("메모리 할당 실패");
    //     }
        
    //     // ptr과 동일한 주소를 가지는 포인터 변수 val
    //     let val : *mut i32 = ptr;

    //     //val 포인터가 가리키는 메모리 위치에 정수 123을 저장
    //     *val = 123;

    //     // 포인터를 통해 할당된 메모리의 값을 출력

    //     println!("*ptr={}", *ptr);

    //     // libc::free를 이용해 이전에 할당된 메모리를 해제
    //     libc::free(ptr as *mut libc::c_void);
    // }

    // 문자열 끝에 null 종료 문자 추가하고 c 문자열로 변환
    // let message = "Hello Rust\0".as_ptr() as *const libc::c_char;

    // unsafe {
    //     libc::printf(message);
    // }
}
