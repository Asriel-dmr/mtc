use std::{
    env,
    fs::{self, read, File},
    io::Read,
    process,
};
//https://doc.rust-lang.org/book/ch12-00-an-io-project.html - I/O general info
//https://doc.rust-lang.org/rust-by-example/std_misc.html - more general info
//https://doc.rust-lang.org/book/ch05-01-defining-structs.html - structs info
//https://stackoverflow.com/questions/25410028/how-to-read-a-struct-from-a-file-in-rust - maybe read byte file
//https://doc.rust-lang.org/std/io/trait.Read.html ^ same
//https://doc.rust-lang.org/std/fmt/trait.Binary.html ^ same
//https://stackoverflow.com/questions/53826371/how-to-create-a-binary-file-with-rust - maybe write byte file
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        eprintln!("Usage: cargo run -- mtc  (e , d) (full txt / mtc file path) (key_file)");
        process::exit(1);
    }
    let files = check_for_valid_file_input(args);
    let mut read_file = &files[0];
    let mut key_file = &files[1];
}

fn check_for_valid_file_input(args: Vec<String>) -> [File; 2] {
    if &args[3][4..] != ".txt" {
        eprintln!("Please enter a valid key file");
        process::exit(1);
    }
    if !(&args[2][4..] == ".txt" && &args[1] == "e")
        || !(&args[1] == "d" && &args[2][4..] == ".mtc")
    {
        eprintln!("Please enter a valid key file");
        process::exit(1);
    }
    let read_file = match File::open(&args[2]) {
        Err(why) => panic!("couldn't open {}: {}", &args[2], why),
        Ok(file) => file,
    };
    let key_file = match File::open(&args[3]) {
        Err(why) => panic!("couldn't open {}: {}", &args[2], why),
        Ok(file) => file,
    };

    return [read_file, key_file];
}
