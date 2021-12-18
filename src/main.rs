#![allow(unused_variables)]
use rust_arxiv_podcast::{arxiv};
use rust_arxiv_podcast::{fast_speech};


#[tokio::main]
async fn main() {
    
    let processor = fast_speech::PhonemsProcessor::build(
        "./resources/librispeech-lexicon.txt"
    );

    println!("{}", processor.len());
    println!("{}", processor.get("a").unwrap());
    
    let model = fast_speech::load_model("./resources/traced.pt");

    let arxiv_body = match arxiv::get_body().await {
        Ok(body) => body,
        Err(e) => panic!("error in the request of today \t {}", e)
    };

    let feed = arxiv::parse(&arxiv_body[..]).expect(
        "Error in parsing the xlm body"
    );

    println!("{}", feed);
}
