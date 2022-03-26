use select::document::Document;
use select::predicate::Name;
use url::ParseError as UrlParseError;
use url::Url;
mod node;
pub use crate::node::element;

fn main() {
    let url = "http://www.brokenthorn.com/Resources/OSDevIndex.html";
    let samehost = true;
    stat(url, samehost);
}
fn stat(url: &str, samehost: bool) {
    let _ = create_rootnode(url.to_string(), 1, 0, samehost);
    println!("--- Finish ---\n");
}
fn create_rootnode(
    url: String,
    depth: usize,
    crntdepth: usize,
    samehost: bool,
) -> eyre::Result<()> {
    let root = node::element::Link::new(url.clone());
    let mut rootlinks = node::element::Links::new(root);

    let response = reqwest::blocking::get(&url)?;
    let base_url = response.url().clone();
    let docstr = response.text()?;
    let doc = Document::from(docstr.as_str());
    let mut hrefvec: Vec<String> = vec![];

    for href in doc.find(Name("a")).filter_map(|a| a.attr("href")) {
        match Url::parse(href) {
            Ok(url) => {
                hrefvec.push(url.to_string());
            }
            Err(UrlParseError::RelativeUrlWithoutBase) => {
                let url = base_url.join(href)?;
                hrefvec.push(url.to_string());
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }

    hrefvec.sort();
    hrefvec.dedup();
    hrefvec.iter().for_each(|x| {
        rootlinks.add_link(x.to_string(), samehost);
    });

    loop {
        if rootlinks.inc() && crntdepth < depth {
            println!("OK depth:{} = {}", crntdepth, rootlinks.curent_url());
            // let _ = create_rootnode(rootlinks.curent_url(), depth, crntdepth+1, samehost);
        } else {
            break;
        }
    }

    Ok(())
}
