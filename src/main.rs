mod lazy;
mod loading;
mod modeling;

use std::io::Write;

use loading::load_on_disk;
use modeling::LlamaForCausalLM;

fn get_prompt_from_cli() -> String {
    let mut user_input = String::new();
    std::print!("> ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut user_input).unwrap();
    user_input
}

fn main() {
    println!("Loading tokenizer...");
    println!("Downloading from huggingface...");
    println!("Loading model weights...");

    let root = "./model";
    let llama = load_on_disk(root);

    loop {
        let prompt = get_prompt_from_cli();
        // let tokens = tokenize();
        println!("I am bob");
    }
}
