use nlpo3::tokenizer::newmm::NewmmTokenizer;
use nlpo3::tokenizer::tokenizer_trait::Tokenizer;

// fn main() {
    
//     let tokenizer = NewmmTokenizer::new("words_th.txt");
//     let tokens = tokenizer.segment("ห้างสรรพสินค้าเซ็นทรัล เริ่มมีชื่อเสียง และเป็นที่รู้จักจากความมุ่งมั่น", true, false).unwrap();
//     // format!("{:?}", tokens); 
//     for token in tokens {
//         println!("{}", token);
//     }
// }

fn main() {
    let input = "ห้างสรรพสินค้าเซ็นทรัล เริ่มมีชื่อเสียง และเป็นที่รู้จักจากความมุ่งมั่น";

    // Call the get_token function with the input
    let tokens = get_token(input);

    // Print the tokens
    for token in tokens {
        println!("{}", token);
    }
}

fn get_token(input: &str) -> Vec<String> {
    // Create a tokenizer
    let tokenizer = NewmmTokenizer::new("words_th.txt");

    // Tokenize the input
    let tokens = tokenizer.segment(input, true, false).unwrap();

    // Collect tokens into a Vec<String>
    // let token_strings: Vec<String> = tokens.iter().map(|token| token.text.clone()).collect();

    // token_strings
    format!("{:?}", tokens);
}