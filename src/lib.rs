// const PATH_PICTURE_CACHE: String = String::from("");
// /**
// 1.  将本地图片上传到Gitee
//         1. 图片本地缓存地址
//         2.
// 2.  在将图片
//
// //https://gitee.com/fcy111/my-data/raw/master/
// ![image-20220129213536696](C:\Users\樊晨阳1\AppData\Roaming\Typora\typora-user-images\image-20220129213536696.png)
//  **/
// 1.push本地缓存的图片
// 2.替换图片路径
// 3.将新的内容输出到指定文件

use std::{env, process, thread};
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::process::Command;

pub mod tests;

const PATH_CACHE: &str = "C:\\Users\\樊晨阳1\\AppData\\Roaming\\Typora\\typora-user-images\\";
const DRAWING_BED_POSITION: &str = "https://gitee.com/fcy111/my-data/raw/master/";

pub fn begin_task(task: TaskUnit) {
    let handle = thread::spawn({
        || {
            change_file(task);
        }
    });
    push_picture_cache();
    handle.join().unwrap();
}

#[derive(Debug)]
pub struct TaskUnit {
    pub source_path: String,
    pub target_path: String,
}

impl TaskUnit {
    fn new_task_two(p1: &String, p2: &String) -> TaskUnit {
        TaskUnit {
            source_path: String::from(p1),
            target_path: String::from(p2),
        }
    }

    fn new_error() -> TaskUnit {
        println!("参数不合法");
        process::exit(2);
    }
}

fn rename_path(path: &String) -> String {
    let chars = path.chars();
    let mut new: Vec<char> = Vec::new();
    for x in chars {
        if x == '.' {
            new.push('_');
        }
        new.push(x);
    }
    return new.iter().collect();
}

pub fn parse_parameter() -> TaskUnit {
    // read the environment variable
    // demo: cargo run source.txt target.txt
    let args: Vec<String> = env::args().collect();
    let task = match args.len() {
        2 => {
            TaskUnit::new_task_two(&args[1], &rename_path(&args[1]))
        }
        3 => {
            TaskUnit::new_task_two(&args[1], &args[2])
        }
        _ => {
            TaskUnit::new_error()
        }
    };
    return task;
}

pub fn change_file(task: TaskUnit) {
    println!("{:?}", task);
    let source_file = File::open(task.source_path).unwrap();
    let buff_reader = BufReader::new(source_file);
    let new_file = File::create(task.target_path).unwrap();
    let mut buff_writer = BufWriter::new(new_file);
    for line in buff_reader.lines() {
        let con = line.unwrap();
        let new_con: String;
        new_con = if con.contains(PATH_CACHE) {
            con.replace(PATH_CACHE, &String::from(DRAWING_BED_POSITION))
        } else {
            con
        };
        buff_writer.write_all(new_con.as_bytes()).unwrap();
    }
    buff_writer.flush().unwrap();
}

const COMMIT_INFO: &str = "commit by mdhelper auto commit";

fn push_picture_cache() {
    // 依次执行命令
    // Command::new("cd").arg(PATH_CACHE).output().unwrap();
    let output = Command::new("cmd").arg("/c")
        .current_dir(PATH_CACHE)
        .arg("git add .")
        .output().unwrap();
    // .args(["git commit", "-m", COMMIT_INFO, "git pull", "git push"])
    Command::new("git").current_dir(PATH_CACHE).args(["commit", "-m", COMMIT_INFO]).output().unwrap();
    Command::new("git").current_dir(PATH_CACHE).args(["pull", "origin", "master"]).output().unwrap();
    Command::new("git").current_dir(PATH_CACHE).arg("push").output().unwrap();
    let content = String::from_utf8_lossy(&output.stdout);
    println!("{}", content)
}