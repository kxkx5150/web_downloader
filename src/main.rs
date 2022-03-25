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
    create_node(url).await?;
    Ok(())
}

async fn create_node(url: &str) -> Result<()> {
    let s = reqwest::get(url.to_string()).await?.text().await?;
    let root = node::element::Link::new(url.to_string());
    let mut rootlinks = node::element::Links::new(root);
    let mut tmplist: Vec<String> = vec![];

    Document::from(s.as_str())
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .for_each(|x| {
            tmplist.push(x.to_string());
        });

    tmplist.sort();
    tmplist.dedup();
    tmplist.iter().for_each(|x| {
        rootlinks.add_link(x.to_string());
    });

    rootlinks.print_links();
    Ok(())
}
