use std::io;

pub fn input_num() -> u32 {
  let mut input_str = String::new();
  io::stdin().read_line(&mut input_str).expect("Failed to read line.");
  convert_from_string_to_u32(&input_str)
}

fn convert_from_string_to_u32(input_str: &str) -> u32 {
  return input_str.trim().parse::<u32>().expect("`input_str` is only accepted for strings of numbers.");
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn strings_of_numbers_should_convert_to_number() {
      assert_eq!(32, convert_from_string_to_u32("32"));
  }

  #[test]
    #[should_panic(expected = "`input_str` is only accepted for strings of numbers.")]
  fn strings_should_not_convert_to_number() {
      convert_from_string_to_u32("test");
  }
}
