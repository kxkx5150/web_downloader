use select::document::Document;
use select::predicate::Name;

mod node;
pub use crate::node::element;

fn main() {
    let url = "http://www.brokenthorn.com/Resources/OSDevIndex.html";
    stat(url);
}
fn stat(url: &str) {
    let _ = create_rootnode(url.to_string(), 1, 0);
    println!("--- Finish ---\n");
}
fn create_rootnode(url: String, depth: usize, crntdepth: usize) -> eyre::Result<()> {
    let s = reqwest::blocking::get(&url)?.text()?;
    let root = node::element::Link::new(url.clone());
    let mut rootlinks = node::element::Links::new(root);
    let mut hrefvec: Vec<String> = vec![];
    let doc = Document::from(s.as_str());

    doc.find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .for_each(|x| {
            hrefvec.push(x.to_string());
        });
    hrefvec.sort();
    hrefvec.dedup();
    hrefvec.iter().for_each(|x| {
        rootlinks.add_link(x.to_string());
    });

    loop {
        if rootlinks.inc() && crntdepth < depth {
            println!("OK depth:{} = {}", crntdepth, rootlinks.curent_url());
            let _ = create_rootnode(rootlinks.curent_url(), depth, crntdepth+1);
        } else {
            break;
        }
    }

    Ok(())
}
