use std::{fs::{create_dir, create_dir_all, remove_dir, remove_dir_all, rename}, path::{Path, PathBuf}};

fn main() {
    // Folder Handling

    // Do not forget to control that paths exists or not in practice

    // Create
    create_dir("/home/mortwain/Downloads/test").unwrap();
    create_dir_all("/home/mortwain/Downloads/tests/a/b").unwrap();
    
    let p:&Path = Path::new("/home/mortwain/Downloads/data");
    create_dir(p).unwrap();

    let mut pb:PathBuf = PathBuf::new();
    pb.push("/home/mortwain/Downloads/data2/a/b");
    create_dir_all(pb).unwrap();


    // Remove
    remove_dir("/home/mortwain/Downloads/test").unwrap();
    remove_dir_all("/home/mortwain/Downloads/tests").unwrap();

    let p:&Path = Path::new("/home/mortwain/Downloads/data");
    remove_dir(p).unwrap();

    let pb:PathBuf = PathBuf::from("/home/mortwain/Downloads/data2");
    remove_dir_all(pb).unwrap();


    // Rename (U can also use it for move a folder to new path)
    let mut pb:PathBuf = PathBuf::from("/home/mortwain/Downloads/test");

    create_dir(&pb).unwrap();
    rename("/home/mortwain/Downloads/test", "/home/mortwain/Downloads/new_test").unwrap();

    pb.set_file_name("new_test");
    let mut pb2:PathBuf = pb.clone();
    pb2.pop();
    pb2.push("nnn");
    rename(&pb, &pb2).unwrap();

    remove_dir(&pb2).unwrap();

    let p:&Path = Path::new("/home/mortwain/Downloads/tt"); 
    // I made it manually, there is also .txt file in tt folder.
    let p2:&Path = Path::new("/home/mortwain/Documents/test_new");

    rename(&p, &p2).unwrap();
}
