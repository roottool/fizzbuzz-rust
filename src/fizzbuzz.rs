pub fn range_fizzbuzz(arg: i32) {
    let max = if arg > 0 { arg } else { 100 };

    for num in 1..=max {
        if num % 3 == 0 && num % 5 == 0 {
            println!("fizzbuzz");
        } else if num % 3 == 0 {
            println!("fizz");
        } else if num % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", num);
        }
    }
}
