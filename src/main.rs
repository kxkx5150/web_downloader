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
    stat(url).await?;
    Ok(())
}
async fn stat(url: &str) -> Result<()> {
    create_node(url, 2, 0, 0).await?;
    Ok(())
}
async fn create_node(url: &str, depth: usize, mut crntdepth: usize, mut idx: usize) -> Result<()> {
    if depth == crntdepth {
        println!("Finish");
        return Ok(());
    }
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

    if rootlinks.len() == idx + 1 {
        println!("Finish");
    } else {
        idx += 1;
        // let _ = create_node(url, depth, crntdepth, idx);
    }

    Ok(())
}
