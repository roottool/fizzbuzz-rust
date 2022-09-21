mod fizzbuzz;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Arguments", about = "An argument of fizzbuzz.")]
struct Opt {
    /// A max value of fizzbuzz execution.
    #[structopt(short = "m", long = "max", default_value = "100")]
    max: i32,
}

fn main() {
    let opt = Opt::from_args();
    fizzbuzz::range_fizzbuzz(opt.max);
}
