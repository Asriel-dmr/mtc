use std::env;
//use std::thread;
    //use std::fs;
    //use std::time::Duration;
    //https://doc.rust-lang.org/std/fs/struct.File.html
    //https://doc.rust-lang.org/book/ch16-01-threads.html
    //https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { //find a better way to check the len
		println!("Please enter the path for the input, key and output files (e/d input_path output_path key_path)");
	}

	let  response = 0; //make mutable later.

	if args[1] == "e" {
		//response = 	huff_encode_ascii_file(args[2], args[3], args[4]);
	}else{
		//response = 	huff_decode_ascii_file(args[2], args[3], args[4]);
	}
    if response < 0{
		println!("operation failed.");
	}

}
//best example so far
//https://github.com/ludlows/chuffman/blob/master/huff_encode.c
//use that one ( at least so far) and change it abit to work with threads.
//https://www.programiz.com/dsa/huffman-coding
//https://www.geeksforgeeks.org/huffman-coding-greedy-algo-3/
//https://github.com/bhrigu123/huffman-coding
//https://github.com/drichardson/huffman
//https://github.com/e-hengirmen/Huffman-Coding
//https://github.com/topics/huffman-coding
