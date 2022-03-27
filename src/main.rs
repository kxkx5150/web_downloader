use select::document::Document;
use select::predicate::Name;
use url::ParseError as UrlParseError;
use url::Url;
mod node;
mod options;
pub use crate::node::element;
pub use crate::options::dl_options;

fn main() {
    let url = "http://www.brokenthorn.com/Resources/OSDevIndex.html";
    let opts = options::dl_options::Options::new(
        2, 
        true
    );
    stat(url, opts);
}
fn stat(url: &str, opts: options::dl_options::Options) {
    let _ = create_rootnode(url.to_string(), 0, &opts);
    println!("--- Finish ---\n");
}
fn create_rootnode(
    url: String,
    crntdepth: usize,
    opts: &options::dl_options::Options,
) -> eyre::Result<()> {
    let root = node::element::Link::new(url.clone());
    let mut rootlinks = node::element::Links::new(root);

    let response = reqwest::blocking::get(&url)?;
    let base_url = response.url().clone();
    let docstr = response.text()?;
    let doc = Document::from(docstr.as_str());

    check_link(doc, base_url, &mut rootlinks, &opts);

    loop {
        if rootlinks.inc() && crntdepth < opts.depth {
            println!("OK depth:{} = {}", crntdepth, rootlinks.curent_url());
            // let _ = create_rootnode(rootlinks.curent_url(), depth, crntdepth+1, samehost);
        } else {
            break;
        }
    }

    Ok(())
}
fn check_link(
    doc: Document,
    base_url: Url,
    rootlinks: &mut node::element::Links,
    opts: &options::dl_options::Options,
) {
    let mut hrefvec: Vec<String> = vec![];

    for href in doc.find(Name("a")).filter_map(|a| a.attr("href")) {
        match Url::parse(href) {
            Ok(url) => {
                hrefvec.push(url.to_string());
            }
            Err(UrlParseError::RelativeUrlWithoutBase) => {
                let url = base_url.join(href).unwrap();
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
        rootlinks.add_link(x.to_string(), opts.samehost);
    });
}
// fn download_page(){

// }