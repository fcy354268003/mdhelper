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

use std::{fs, path};
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Error, Write};
use std::path::Path;
use std::process::Command;

pub mod tests;

const PATH_CACHE: &str = "C:\\Users\\樊晨阳1\\AppData\\Roaming\\Typora\\typora-user-images\\";

pub fn begin_task(path: &String, new_path: &String) {
    //
    push_picture_cache();
    read_from_source(path, new_path);
}

pub fn read_from_source(path: &String, new_path: &String) {
    let source_file = File::open(path).unwrap();
    let buff_reader = BufReader::new(source_file);
    let new_file = File::create(new_path).unwrap();
    let mut buff_writer = BufWriter::new(new_file);
    for line in buff_reader.lines() {
        let con = line.unwrap();
        let new_con: String;
        new_con = if con.contains(PATH_CACHE) {
            con.replace(PATH_CACHE, &String::from("https://gitee.com/fcy111/my-data/raw/master/"))
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

