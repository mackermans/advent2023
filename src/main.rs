use std::env;

mod day1;
mod day2;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = args[1].as_str();

    match day {
        "day1" => day1::run(),
        "day2" => day2::run(),
        _ => {
            panic!("Specify a day to run, e.g. 'day1'");
        }
    }
}
