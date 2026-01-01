// 두 개의 i32 숫자를 더하고 더한 결과를 반환합니다.
pub fn add(x: i32, y: i32) -> i32 {
  x + y
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
  }
}
