use error_chain::error_chain;
use select::document::Document;
use select::predicate::Name;
mod node;
pub use crate::node::element;

error_chain! {
      foreign_links {
          ReqError(reqwest::Error);
          IoError(std::io::Error);
      }
}

#[tokio::main]
async fn main() -> Result<()> {
    let url = "https://www.yahoo.co.jp/";
    let res = reqwest::get(url.to_string())
        .await?
        .text()
        .await?;
    create_root(url, res.as_str());
    Ok(())
}

fn create_root(url: &str, s: &str) {

    let root = node::element::Link::new(url.to_string());
    let rlink = node::element::Links::new(root);


    let mut as_str: Vec<String> = Vec::new();

    Document::from(s)
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .for_each(|x| {
            if x.starts_with("http") {
                as_str.push(x.to_string());
            }
        });

    // as_str.sort();
    // as_str.dedup();
    // println!("{:?}", as_str);
}

