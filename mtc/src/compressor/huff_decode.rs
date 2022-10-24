
use std::fs::File;
use std::io::Write;
// define the structure of huffman decode node
struct huff_decode_node {
	pub left: Option<Box<huff_decode_node>>,  // represents 0
	pub right: Option<Box<huff_decode_node>>, // represents 1
	pub is_leaft: u32,
	pub smybol: char,
}


impl huff_decode_node{
	fn create_huff_decode_node() -> huff_decode_node {
		huff_decode_node{left:NULL,right:NULL,is_leaf:false,smybol:NULL}
	}
	// function to free the memory allocated to the decode tree
	// fn free_huff_decode_tree(HDecodeNode* node) {
	// 	if (node == NULL) return;
	// 	free_huff_decode_tree(node->left);
	// 	free_huff_decode_tree(node->right);
	// 	free(node);
	// 	node = NULL;
	// }


	//function to build huffman decode tree based on codebook.
	// the format of codebook should be like:
	// a 10000
	// b 10001
	// ....
	// for each symbol, it creates nodes from the top to the bottom
	fn build_huff_decode_tree(fp: File , root_decode: huff_decode_node) {// fp = key file i think, change to file from &str
		let mut symbol = NULL;
		let mut strcode = vec![];
		let mut length=0;
		let mut num_input=0;
		let mut curr_node = NULL;

		let mut contents = vec![];
		let mut contents_symbol = vec![];
		let mut contents_strcode = vec![]; // make it a vector of vector
		let mut temp =  vec![];
		f_in.read_to_end(&mut contents)?;
		for c in contents{
			temp.push(c);
			if c == '\n'{
				contents_symbol.push(temp[1]);
				contents_strcode.push(temp[4..]); 
				println!(temp[4..]);
				let len = len(temp);
				temp.clear();
				temp.resize(len, false);
			}
			//reset vector temp
			
		}
		for i in len(contents_symbol){//make it inrange
			strcode = contents_strcode[i];
			symbol = contents_symbol[i];

			if (num_input != 2) {
				break;
			}
			println!("reading:({}{})successfully", symbol, strcode); 
			// point curr_node to the root
			curr_node = root_decode;
			length = len(strcode);
			println!("length={}", length); 
			// create nodes one by one
			for idx in length {
				if (strcode[idx] == '0') {
					if (curr_node.left == NULL) {
						curr_node.left = create_huff_decode_node();
					}
					curr_node = curr_node.left;
				}
				else if(strcode[idx] == '1') { // strcode[idx] == '1'
					if (curr_node.right == NULL) {
						curr_node.right = create_huff_decode_node();
					}
					curr_node = curr_node.right;
				}
				else {
					println!("unexpected char {}", strcode[idx]); 
					assert(0); 
				}
			}

			assert_eq!(curr_node.is_leaf, 0);
			// at last assign the symbol to the leaf node
			curr_node.is_leaf = 1;
			curr_node.symbol = symbol;
			println("successfully inserted symbol:{}{}", symbol, strcode); 
		}
	}


	// function to decode
	// f_in is the file pointer to encoded file
	// f_out is the file pointer to write decoded message
	fn huff_decode(f_in: File, f_out: File, root_decode: huff_decode_node) { //change to files from &str
		let mut c = NULL;
		// int num_read = 0;
		// assign the current node to root of decode tree
		let mut curr_node = root_decode;
		let mut contents = vec![];
		let mut new_contents = vec![];

		f_in.read_to_end(&mut contents)?;
		for c in contents{ // updating c bit by bit (mimic) //change to stop until file ends.
			// num_read = fscanf_s(f_in, "%c", &c, 1);
			if (c == '0') {
				curr_node = curr_node.left;
			}
			else if (c == '1') {  // c == '1'
				curr_node = curr_node.right;
			}
			else {
				println!("\nchar{}rather than 1 or 0 appears", c);//change
				assert(0); // raise error;
			}
			if (curr_node.is_leaf) {
				//printf("%c", curr_node->symbol);
				new_contents.push(curr_node.symbol); //change to write into file.
				// need to assign the curr_node to root_decode to start over
				curr_node = root_decode; 
			}
			f_out.write(b"{}",new_contents)?;
		}
	}

	// interface to decode a file 
	fn huff_decode_ascii_file(&str: filename, &str: codebook_filename, &str: decoded_filename) {
		// decode
		// get the root to decode
		//let mut f_in = NULL;
		//let mut f_out = NULL;
		
		// fopen_s(&f_in, "codebook.txt", "r");
		let mut f_in = File::open(codebook_filename); 
		if(f_in == NULL){
			println!("cannot open {}\n.exit.", codebook_filename); //change
			return -1;
		}

		let mut root_decode = create_huff_decode_node(); //change
		build_huff_decode_tree(f_in, root_decode); 

		drop(f_in); 
		f_in = NULL;
		

		// fopen_s(&f_in, "encoded.txt", "r");
		f_in = File::open(filename); 
		if(f_in == NULL){
			println!("cannot open {}\n.exit.", filename);
			return -1;
		}
		
		// fopen_s(&f_out, "decoded.txt", "w");
		f_out = File::open(decoded_filename);
		if(f_out == NULL){
			println!("cannot open {}\n.exit.", decoded_filename);
			return -1;
		}
		huff_decode(f_in, f_out, root_decode);
		// clean memory on the heap
		//free_huff_decode_tree(root_decode);
		return 0;
	}
}



