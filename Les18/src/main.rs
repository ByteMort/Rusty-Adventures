use std::path::{Path, PathBuf};


fn main() {
    // Path =>  Immutable, Just Read and Reference, (Works like &str)
    // PathBuf  =>  Mutable, Make new Path and Change it, (Works like String)

    // Path
    let p:&Path = Path::new("/home/mortwain/Downloads");

    // Control
    println!("Exists?: {}", p.exists());
    println!("Is File?: {}", p.is_file());
    println!("Is Directory?: {}", p.is_dir());

    // Info
    println!("File Name: {}", p.file_name().unwrap().to_string_lossy());
    // println!("Extension: {}", p.extension().unwrap().to_string_lossy());
    println!("Parent: {}", p.parent().unwrap().to_string_lossy());
    println!("File Stem: {}", p.file_stem().unwrap().to_string_lossy());
    
    // Absolute or Relative
    println!("Is Absolute? : {}", p.is_absolute());
    println!("Is Relatieve? : {}", p.is_relative());

    // Part of path
    for part in p.iter(){
        println!("{:?}", part);
    }

    // Str, Display
    println!("Str: {:?}", p.to_str());
    println!("Display: {}", p.display());

    // Comparison
    let p2: &Path = Path::new("../../Downloads");
    println!("P1 and p2: {}", p==p2);

    // PathBuf
    let mut pb:PathBuf = PathBuf::new();
    pb.push("Data");
    pb.push("file.txt");
    // let mut pb:PathBuf = PathBuf::from("Data/file.txt");

    println!("PathBuf: {}", pb.display());

    pb.pop();
    println!("PathBuf: {}", pb.display());

    pb.set_file_name("new_name");
    println!("PathBuf: {}", pb.display());

    pb.set_extension("md");
    println!("PathBuf: {}", pb.display());

    // Path to PathBuf
    let p3:&Path = Path::new("/home/mortwain/Downloads");
    println!("Exists? : {}", p3.exists());
    let file_path:PathBuf = p3.join("file.txt");
    println!("Exists? : {}", file_path.exists());

    let mut file_path2:PathBuf = p3.to_path_buf();
    file_path2.push("file.txt");
    println!("Exists? : {}", file_path2.exists());

    // PathBuf to Path
    let p4:&Path = &file_path2;
    println!("Exists? : {}", p4.exists());
}
