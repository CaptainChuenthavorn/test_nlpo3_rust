// use std::io::Read;
use nlpo3::tokenizer::newmm::NewmmTokenizer;
use nlpo3::tokenizer::tokenizer_trait::Tokenizer;
use std::path::Path;
// use std::fs::File;
use std::fs;
use std::env;

// pub fn main() {
//     match env::current_dir() {
//         Ok(current_dir) => {
//             match current_dir.read_dir() {
//                 Ok(entries) => {
//                     for entry in entries {
//                         match entry {
//                             Ok(dir_entry) => {
//                                 if let Some(file_name) = dir_entry.file_name().to_str() {
//                                     println!("{}", file_name);
//                                 } else {
//                                     eprintln!("Error converting file name to string");
//                                 }
//                             }
//                             Err(e) => {
//                                 eprintln!("Error reading directory entry: {}", e);
//                             }
//                         }
//                     }
//                 }
//                 Err(e) => {
//                     eprintln!("Error reading directory: {}", e);
//                 }
//             }
//         }
//         Err(e) => {
//             eprintln!("Error getting current directory: {}", e);
//         }
//     }
// }


fn main() {
    
    
    // let mut file = File::open("train.txt").expect("Can't open file!");
    // let mut contents = String::new();
    // file.read_to_string(&mut contents).expect("pps!Can not read the file...");
    // println!("File contents\n\n{}",contents);
    println!("{}", Path::new("../src/word_th.txt").exists());
    let tokenizer = NewmmTokenizer::new("words_th.txt");
    let words: Vec<String> = vec!["เลขที่อ้างอิง".to_string(), "คอนสติติวชั่น".to_string()];
    let tokenizer = NewmmTokenizer::from_word_list(words);
    let tokens = tokenizer.segment("เลขที่อ้างอิง : 928471812", true, false).unwrap();
    
    // let tokenizer = NewmmTokenizer::new(&relative_dict_path);
    // let result = tokenizer.segment(&text, false, true).unwrap();
    // let safe_result = tokenizer.segment(&text, true, true).unwrap();
    // format!("{:?}", tokens); 
    for token in tokens {
        println!("{}", token);
    }
}

// fn main() {
//     let input = "ห้างสรรพสินค้าเซ็นทรัล เริ่มมีชื่อเสียง และเป็นที่รู้จักจากความมุ่งมั่น";

//     // Call the get_token function with the input
//     let tokens = get_token(input);

//     // Print the tokens
//     for token in tokens {
//         println!("{}", token);
//     }
// }

// fn get_token(input: &str) -> Vec<String> {
//     // Create a tokenizer
//     let tokenizer = NewmmTokenizer::new("words_th.txt");

//     // Tokenize the input
//     let tokens = tokenizer.segment(input, true, false).unwrap();

//     // Collect tokens into a Vec<String>
//     // let token_strings: Vec<String> = tokens.iter().map(|token| token.text.clone()).collect();

//     // token_strings
//     format!("{:?}", tokens);
// }