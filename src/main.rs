use rust_arxiv_podcast::{arxiv};
use rust_arxiv_podcast::{fast_speech};

#[tokio::main]
async fn main() {

    let model = fast_speech::load_model().expect(
        "error loading the model"
    );

    let arxiv_body = arxiv::get_body().await.expect(
        "error in the request of today"
    );

    let feed = arxiv::parse(&arxiv_body[..]).expect(
        "Error in parsing the xlm body"
    );

    println!("{}", feed);
}
