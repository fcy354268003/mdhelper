use std::time::Instant;
use mdhelper::{begin_task, parse_parameter};

fn main() {
    let current = Instant::now();
    // read_from_source(&String::from("d:\\test.md"), &String::from("d:\\newTest.md"));
    let task = parse_parameter();
    println!("\n\n{:?}", task);
    // let target_path = &task.target_path;
    begin_task(&task);
    // File::open("C:\\Users\\樊晨阳1\\Desktop\\jps_V.txt").unwrap();
    println!("执行时间:{} ms \n输出文件:{}", current.elapsed().as_millis(), task.target_path);
}


