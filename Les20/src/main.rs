use std::{fs::{File, OpenOptions, ReadDir, copy, create_dir, read_dir, remove_file, rename}, io::{ErrorKind, IoSlice, IoSliceMut, Read, Seek, Write}, os::unix::fs::FileExt, path::{Path, PathBuf}, vec};

fn main() {
    // File Handling
    
    // let p:&Path = Path::new("./src/myTextFile.txt");
    // File::create(&p).unwrap(); // if it doesn't exist, create it; if it exists, overwrite it.
    // File::create_new(&p).unwrap(); // if it doesn't exist, create it; if it exists, returns error.
    
    /*
    if p.exists(){
        println!("This file exists.");
    }else{
        File::create_new(&p).unwrap();
    }
    */
    /*
    // Create
    match File::create_new(&p) {
        Ok(_) => { // _ is File type
            println!("Created at : {:?}", &p.display());
        },
        Err(e) => {
            println!("This file exists. {}", e);
        }
    }
    
    // Remove
    match remove_file(&p) {
        Ok(_) => { // _ there is no file
            println!("Removed from : {}", &p.display());
        },
        Err(e) => {
            println!("This file doenst exist. {}", e);
        }
    }
    
    // Rename
    let p_new:&Path = Path::new("./src/NewName.txt");
    match rename(&p, &p_new){
        Ok(_) => {
            println!("Renamed from {} to {}", 
            &p.file_name().unwrap().to_string_lossy(),
            &p_new.file_name().unwrap().to_string_lossy());
        },
        Err(e) => {
            println!("This file doesn't exist. {}", e);
        }
    };

    let mut p_move:PathBuf = PathBuf::from("./src/myNewFolder");
    match create_dir(&p_move){
        Ok(_) => {
            println!("Created folder: {}", &p_move.display());
        },
        Err(e) => {
            println!("Folder couldn't be created. {}", e);
        }
    }

    p_move.push("NewName.txt");
    // Move
    
    if !p_move.exists(){
        match rename(&p_new, &p_move){
            Ok(_) => {
                println!("File {} moved to {}", &p_new.display(), &p_move.display());
            },
            Err(e) => {
                println!("File couldnt be moved. {}", e);
            }
        }
    }
    
    // Copy
    let p2:&Path = Path::new("./src/myFile.txt");
    match copy(&p, &p2){
        Ok(_) => {
            println!("Copied to : {}", &p2.display());
        },
        Err(e) => {
            println!("Something went wrong: {}", e);
        }
    }
    */

    /*
    // FS Write Methods
    let mut file = File::create(&p).unwrap();

    // Write
    let total_bytes:usize= file.write("Heyyy".as_bytes()).unwrap();
    file.write("Boom".as_bytes()).unwrap();
    println!("Total bytes: {}", total_bytes);

    // Write All
    let result = file.write_all("Let's goo".as_bytes()).unwrap();
    println!("{:?}", result);

    // Write Format
    let name:String = String::from("Mortwain");
    file.write_fmt(format_args!("Hello {}", name)).unwrap();

    write!(file, "Hello {}", "Bytemort").unwrap();

    // Write At and Write All At from 5 byte of file
    file.write_at("UHHHMM".as_bytes(), 5).unwrap();
    file.write_all_at("Oghh".as_bytes(), 3).unwrap();

    // Write Vectored
    let my_bufs = &[
        IoSlice::new("XDD".as_bytes()),
        IoSlice::new("me".as_bytes())
        ];
    file.write_vectored(my_bufs).unwrap();
    */

    /*
    // FS Open en Read Methods
    let mut file:File = File::open(&p).unwrap();

    // Reads bytes from the file. (It reads 15 bytes from file) 
    let mut buffer:[u8;15] = [0;15];
    let bytes_read:usize = file.read(&mut buffer).unwrap();
    println!("Readed bytes {} - Text {:?}", bytes_read, buffer);

    // Take offset from beginning of file and read bytes
    let mut buffer:[u8;15] = [0;15];
    let bytes_read:usize = file.read_at(&mut buffer, 2).unwrap();
    println!("Readed bytes {} - Text {:?}", bytes_read, buffer);

    // Read from where we left off last time.
    let mut text:String = String::new();
    // let mut file:File = File::open(&p).unwrap(); // => if u open then u start from top of file.
    // file.seek(std::io::SeekFrom::Start(0)).unwrap();
    // file.seek_relative(5).unwrap(); // same as seek Current
    let bytes_read:usize = file.read_to_string(&mut text).unwrap();
    println!("Readed bytes {} - Text {}", bytes_read, text);

    // Read from where we left off last time.
    let mut my_vec:Vec<u8> = Vec::new();
    file.seek(std::io::SeekFrom::Start(0)).unwrap();
    let bytes_read:usize= file.read_to_end(&mut my_vec).unwrap();
    println!("Readed bytes {} - Text {:?}", bytes_read, my_vec);


    let mut buffer:[u8;2] = [0;2];
    file.seek(std::io::SeekFrom::Start(0)).unwrap();
    let result = file.read_exact(&mut buffer).unwrap();
    println!("Readed {:?} - Text {:?}", result, buffer);

    let mut buffer:[u8;3] = [0;3];
    let result = file.read_exact_at(&mut buffer, 4).unwrap();
    println!("Readed {:?} - Text {:?}", result, buffer);


    let mut buf1:[u8;2]= [0;2];
    let mut buf2:[u8;2] = [0;2];
    let mut vectored:[IoSliceMut<'_>; 2] = [
        IoSliceMut::new(&mut buf1),
        IoSliceMut::new(&mut buf2),
    ];

    let bytes_read:usize = file.read_vectored(&mut vectored).unwrap();
    println!("Readed bytes {} - Text {:?}", bytes_read, vectored);
    */

    /*
    // OpenOptions
    let mut my_file:File = OpenOptions::new()
        .read(true)
        .write(true)
        //.append(true)
        .create(true)
        .truncate(true)
        .open(&p).unwrap();

    println!("File: {:?}", my_file);
    
    my_file.write_all("\nHello Mortwain!".as_bytes()).unwrap();
    */

    /*
    // Read Dir
    let dir_path:&Path = Path::new("./src/myFolder");
    let readed_dir:ReadDir = read_dir(&dir_path).unwrap();
    
    for entry in readed_dir{
        let entry = entry.unwrap();
        let path = entry.path();
        let file_name = entry.file_name();
        // let meta_data = entry.metadata().unwrap();

        println!("Entry: {:?}\nPath: {}\nFileName: {}\n",
            entry, path.display(), file_name.display());
    }
    */
}
