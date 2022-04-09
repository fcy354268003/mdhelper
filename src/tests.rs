use super::*;


#[test]
fn readFile() {
    change_file(&String::from("d:\\test.md"), &String::from("d:\\newTest.md"));
}

#[test]
fn ss() {
    push_picture_cache()
}