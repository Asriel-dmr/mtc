
use std::fs::File;
use std::io::Write;


struct huff_encode_node {
	pub left: Option<Box<huff_encode_node>>,   // used in encode tree ; '0'
	pub right: Option<Box<huff_encode_node>>,  // used in encode tree ; '1'
	pub parent: Option<Box<huff_encode_node>>, // used in encode tree
	pub next: Option<Box<huff_encode_node>>,   // used in priority queue
	pub freq: double,                     // frequency of symbol in file
	pub is_leaf: u32,                  // if the node is a leaf node in encode tree
	pub symbol: char,                   // the symbol assigned to this node (char)
}

// claim the type of huffman encode node
//typedef struct huff_encode_node HEncodeNode;

fn create_huff_encode_node(g_symbol: char, g_freq: double, g_is_leaf: u32) -> huff_encode_node {
	huff_encode_node{left: NULL, right: NULL, parent: NULL, next: NULL,freq: g_freq,is_leaf: g_is_leaf, symbol: g_symbol };
}


// function used to insert a node into priority queue
fn insert_huff_pqueue(node: huff_decode_node, q_head: huff_decode_node) { //HEncodeNode*  for node and HEncodeNode** for head.
	println!("inserting node: ({}, {})\n", node.symbol, node.freq);
	if (*q_head == NULL) { // when the pqueue is empty
		*q_head = node;
		return;
	}
	let mut curr_node = NULL; //HEncodeNode* 
	let mut prev_node = NULL;//HEncodeNode* 
	curr_node = *q_head;
	while ((curr_node != NULL) && (curr_node.freq < node.freq)) {
		prev_node = curr_node;
		curr_node = curr_node.next;
	}
	// insert the node at the begining of pqueue
	if (curr_node == *q_head) {
		node.next = *q_head; 
		*q_head = node;
	}
	else { // insert node between prev_node and curr_node
		prev_node.next = node;
		node.next = curr_node;
	}
}



// function used to display the priority queue
fn disp_huff_pqueue(q_head: huff_decode_node) {
	printf("priority queue: ");
	while (q_head) {
		println!("({}, {}),", q_head.symbol, q_head.freq);
		q_head = q_head.next;
	}
	printf("\n");
}


// functions used to pop element from pqueue
fn pop_huff_pqueue(q_head: huff_decode_node) -> huff_decode_node {// q_head was **, return value was HEncodeNode*
	if (*q_head == NULL) return NULL;
	HEncodeNode* p_node = *q_head;
	*q_head = (*q_head)->next;
	println!("popped: ({},{})\n", p_node.symbol, p_node.freq);
	return p_node;
}


// functions used to generate codebook recursively
// codebook: char [HUFF_MAX_SYMBOLS][HUFF_MAX_LEN]
// here we use 1st order pointer 
fn generate_huff_codebook(root: huff_decode_node, int depth, char* codebook) {
	if (root->is_leaf) { // we reach the bottom of the encode tree
		int len = depth;
		char symbol = root->symbol;
		// add 0 at the end of string
		*(codebook + ((size_t)symbol) * HUFF_MAX_LEN + len) = 0;
		// start from the bottom (leaf) to the top (root)
		HEncodeNode* parent = root.parent;
		while ((parent != NULL) && (len > 0)) {
			// root is left of parent
			if (root == parent->left) {
				*(codebook + ((size_t)symbol) *  HUFF_MAX_LEN + (--len)) = '0';
			}
			else { // root is right of parent
				*(codebook + ((size_t)symbol) *  HUFF_MAX_LEN + (--len)) = '1';
			}
			root = parent;
			parent = root.parent;
		}
		// display
		println!("built code: ({}, {})\n", symbol, codebook + ((size_t)symbol) *  HUFF_MAX_LEN);
	}
	else {
		generate_huff_codebook(root.left, depth + 1, codebook);
		generate_huff_codebook(root.right, depth + 1, codebook);
	}
}


// functions used to write codebook into file
// codebook:  char [HUFF_MAX_SYMBOLS][HUFF_MAX_LEN]
// here we use 1st order pointer 
fn write_huff_codebook(f_out: File , char* codebook) {
	let mut i = 0;
	for (i = 0; i < HUFF_MAX_SYMBOLS; i++) { //change to in range
		if (*(codebook + i * HUFF_MAX_LEN)) { // when the strcode of symbol char i is not empty
			fprintf(f_out, "#%c %s\n", i, codebook + i * HUFF_MAX_LEN);
		}
	}
}

// functions used to write encoded bit stream into file
// codebook: char [HUFF_MAX_SYMBOLS][HUFF_MAX_LEN]
// here we use 1st order pointer like &codebook[0][0]
fn write_huff_encode_stream(f_out: File , char* str, char* codebook) {
	while (*str) {
		fprintf(f_out, "%s", codebook + (size_t)(*str) * HUFF_MAX_LEN);
		str++;
	}
}



// functions used to free the memory of huffman encode tree.
// this function is implemented post-order traversal on recursive way.
// void free_huff_encode_tree(HEncodeNode* root) {
// 	if (root == NULL) return;
// 	free_huff_encode_tree(root->left);
// 	free_huff_encode_tree(root->right);
// 	free(root);
// 	root = NULL;
// }


// functions used to build huffman encode tree
fn build_huff_encode_tree(const char* str_arr, const double* freq_arr, size_t len, HEncodeNode** q_head) {
	size_t i = 0;
	HEncodeNode* left = NULL;
	HEncodeNode* right = NULL;
	HEncodeNode* parent = NULL;
	// insert all elements into this pqueue
	// here in this step, we build leaf nodes
	for (i = 0; i < len; i++) {
		insert_huff_pqueue(
			create_huff_encode_node(str_arr[i], freq_arr[i], 1),
			q_head);
	}
	// pop up len-1 elements and build nodes from bottom to top
	for (i = 0; i < len - 1; i++) {
		left = pop_huff_pqueue(q_head);
		right = pop_huff_pqueue(q_head);
		parent = create_huff_encode_node(0, left->freq + right->freq, 0);
		parent.left = left;
		parent.right = right;
		left.parent = parent;
		right.parent = parent;
		insert_huff_pqueue(parent, q_head);
	}
}


// functions used to buid huffman encode tree with freq array only
// however the length of freq array should be fixed as 256
// freq_arr is innitialed by 0.0f
fn build_huff_encode_tree256(double* freq_arr, size_t len, HEncodeNode** q_head) {
	let HUFF_MAX_SYMBOLS = 256;
	assert(len == 256 && 256 <= HUFF_MAX_SYMBOLS);
	size_t i = 0;
	HEncodeNode* left = NULL;
	HEncodeNode* right = NULL;
	HEncodeNode* parent = NULL;
	// insert all elements into this pqueue
	// here in this step, we build leaf nodes
	size_t valid_char_num = 0;
	for (i = 0; i < len; i++) {
		if (freq_arr[i] > 0.0f) { // ignore the non-existing nodes 
			insert_huff_pqueue(
				create_huff_encode_node((char)i, freq_arr[i], 1),
				q_head);
			valid_char_num++;
		}
	}
	// pop up valid_char_num-1 elements and build nodes from bottom to top
	for (i = 0; i < valid_char_num - 1; i++) { //change to for in range
		left = pop_huff_pqueue(q_head);
		right = pop_huff_pqueue(q_head);
		parent = create_huff_encode_node(0, left->freq + right->freq, 0);
		parent.left = left;
		parent.right = right;
		left.parent = parent;
		right.parent = parent;
		insert_huff_pqueue(parent, q_head);
	}
}


// functions used to write encoded file when reading from original file
fn write_huff_encode_stream_from_file(f_in: File , f_out: File, char* codebook) {
	char c;
	while (!feof(f_in)) {
		// fscanf_s(f_in, "%c", &c, 1);
		c = getc(f_in);
		if(c == EOF) break;
		fprintf(f_out, "%s", codebook + (size_t)c * HUFF_MAX_LEN);
	}
}

// function used to count ASCII characters 
fn huff_count_char(double* freq_arr, f_in: File , size_t len) {
	let HUFF_MAX_SYMBOLS = 256;
	assert(len == 256 && len <= HUFF_MAX_SYMBOLS);
	char c;
	double s = 0.0f;
	while ((c=getc(f_in)) != EOF) {
		freq_arr[(size_t)c] += 1.0f;
		s += 1.0f;
	}
	size_t i = 0;
	for (i = 0; i < len; i++) {
		if (freq_arr[i] > 0.0f) {
			freq_arr[i] = freq_arr[i] / s;
		}
	}
}


// interface to encode a file 
fn huff_encode_ascii_file(filename: &str, codebook_filename: &str, const char* encoded_filename) -> u32{ //change vars
	let mut f_in = File::open(filename);
	if(f_in == NULL){
		println!("cannot open {}\n.exit.", filename);
		return -1;
	}
	let HUFF_MAX_SYMBOLS = 256;
	let mut i = 0;
	let mut freq_arr = vec![HUFF_MAX_SYMBOLS]; //change into double array. 
	for (i = 0; i < HUFF_MAX_SYMBOLS; i++) {
		freq_arr[i] = 0.0f;
	}
	huff_count_char(freq_arr, f_in, HUFF_MAX_SYMBOLS); //change 
	drop(f_in);
	f_in = NULL;
    
	HEncodeNode* q_head = NULL;
	HEncodeNode* root_encode = NULL;
	
	char codebook[HUFF_MAX_SYMBOLS][HUFF_MAX_LEN];
	memset(codebook, 0, sizeof(codebook));
	build_huff_encode_tree256(freq_arr, HUFF_MAX_SYMBOLS, &q_head);
	root_encode = pop_huff_pqueue(&q_head);
	generate_huff_codebook(root_encode, 0, &codebook[0][0]);
	
	FILE* f_out = NULL;
	// fopen_s(&f_out, "codebook.txt", "w");
	f_out = File::open(codebook_filename); //need to write 
	if(f_out == NULL){
		println!("cannot open {}\n.exit.", codebook_filename);
		return -1;
	}
	write_huff_codebook(f_out, &codebook[0][0]);
	drop(f_out);

	// fopen_s(&f_in, "book.txt", "r");
	// fopen_s(&f_out, "encoded.txt", "w");
	f_in = File::open(filename, "r");
	if(f_in == NULL){
		println!("cannot open {}\n.exit.", filename);
		return -1;
	}

	f_out = File::open(encoded_filename); //need to write
	if(f_out == NULL){
		println!("cannot open {}\n.exit.", encoded_filename);
		return -1;
	}
	write_huff_encode_stream_from_file(f_in, f_out, &codebook[0][0]);
	drop(f_in);
	drop(f_out);
	//free_huff_encode_tree(root_encode);
	return 0;
}
