mod fizzbuzz;
mod arguments;

fn main() {
    let num = arguments::fetch_optional_integer_arg();
    fizzbuzz::range_fizzbuzz(num);
}
