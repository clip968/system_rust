pub fn add(left: u64, right: u64) -> u64 {
    left + right // 두 인수의 합을 반환
}

// cfg(test) 어트리뷰트는 이어지는 코드가 테스트 환경에서만 컴파일 되도록 함
#[cfg(test)]
mod tests {
    use super::*; // 상위 스코프의 모든 항목을 현재 테스트 모듈로 가져온다.
    // 즉 상위 스코프의 add를 가져온다는 의미

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
