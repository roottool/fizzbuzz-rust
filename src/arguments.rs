use std::env;

pub fn fetch_optional_integer_arg() -> i32 {
  let args: Vec<String> = env::args().collect();
  if args.len() == 1 {
    return 0;
  }

  let optional_arg = &args[1];
  convert_from_string_to_i32(optional_arg)
}

fn convert_from_string_to_i32(str_of_num: &str) -> i32 {
  str_of_num.trim().parse::<i32>().expect("`str_of_num` is only accepted for strings of numbers.")
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn strings_of_numbers_should_convert_to_number() {
      assert_eq!(32, convert_from_string_to_i32("32"));
      assert_eq!(-32, convert_from_string_to_i32("-32"));
  }

  #[test]
    #[should_panic(expected = "`str_of_num` is only accepted for strings of numbers.")]
  fn strings_should_not_convert_to_number() {
      convert_from_string_to_i32("test");
  }
}
