use std::env;
//https://doc.rust-lang.org/book/ch12-00-an-io-project.html - I/O general info
//https://doc.rust-lang.org/book/ch05-01-defining-structs.html - structs info
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        println!("Usage: cargo run -- mtc  (e , d) (txt_file) (key_file)");
        return;
    }
    //rest of the code
}
