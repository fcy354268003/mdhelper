use super::*;


#[test]
fn readFile() {
    read_from_source(&String::from("d:\\test.md"), &String::from("d:\\newTest.md"));
}

#[test]
fn ss() {
    push_picture_cache()
}