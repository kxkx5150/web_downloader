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
    create_rootnode(url, 2, 0).await?;
    Ok(())
}
#[allow(unused_variables)]
#[allow(unused_mut)]
async fn create_rootnode(url: &str, depth: usize, mut crntdepth: usize) -> Result<()> {
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

    println!("\n--- start ----");
    let _ = iter_links(rootlinks);
    println!("--- Finish ---\n");
    Ok(())
}
fn iter_links(mut rootlinks: node::element::Links){
    if rootlinks.inc() {
        println!("{}",rootlinks.curent_url());
        let _ = iter_links(rootlinks);
    }
}
