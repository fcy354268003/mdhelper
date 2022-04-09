use std::time::Instant;
use mdhelper::{begin_task, parse_parameter};

fn main() {
    let current = Instant::now();
    // read_from_source(&String::from("d:\\test.md"), &String::from("d:\\newTest.md"));
    begin_task(parse_parameter());
    // File::open("C:\\Users\\樊晨阳1\\Desktop\\jps_V.txt").unwrap();
    println!("执行时间 {}",current.elapsed().as_millis());
}


