/// 두 개의 i32 숫자를 더하고 더한 결과를 반환합니다.
/// ```
/// assert_eq!(chapter_07_listing_06::add(2, 2), 4);
/// ```
///
/// ```
/// use chapter_07_listing_06::add;
/// assert_eq!(add(2, 2), 5);
/// ```
///
/// ```
/// use chapter_07_listing_06::add;
/// assert_eq!(add("hello", 2), 4);
/// ```
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
