#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use core::iter::repeat;
use html5ever::driver::ParseOpts;
use html5ever::rcdom::{Handle, Node, NodeData, RcDom};
use html5ever::serialize;
use html5ever::serialize::SerializeOpts;
use html5ever::tendril::{StrTendril, TendrilSink};
use html5ever::{local_name, namespace_url, ns};
use html5ever::{parse_document, parse_fragment};
use html5ever::{Attribute, LocalName, QualName};
use std::rc::Rc;
use std::cell::RefCell;
use url::Url;
use url::ParseError as UrlParseError;
mod node;
mod options;
pub use crate::node::element;
pub use crate::options::dl_options;

#[allow(unused_variables)]
fn walk(base_url: &Url, indent: usize, node: &Handle, urllinks: &mut node::element::Urllist) {
    match node.data {
        NodeData::Document => {
            // println!("#Document")
        }
        NodeData::Doctype {
            ref name,
            ref public_id,
            ref system_id,
        } => {
            // println!("<!DOCTYPE {} \"{}\" \"{}\">", name, public_id, system_id),
        }
        NodeData::Text { ref contents } => {
            // println!("#text: {}", contents.borrow().escape_default())
        }
        NodeData::Comment { ref contents } => {
            // println!("<!-- {} -->", contents.escape_default()),
        }
        NodeData::Element {
            ref name,
            ref attrs,
            ..
        } => {
            check_tag(base_url, name.local.to_string(), &attrs, urllinks);
        }
        NodeData::ProcessingInstruction { .. } => unreachable!(),
    }

    for child in node.children.borrow().iter() {
        walk(base_url, indent + 4, child, urllinks);
    }
}
#[allow(unused_variables)]
fn check_tag(
    base_url: &Url,
    nodestr: String,
    attrs: &RefCell<Vec<Attribute>>,
    urllinks: &mut node::element::Urllist,
) {
    if nodestr == "a" {
        check_a(base_url, attrs, &mut urllinks.a_links);
    } else if nodestr == "img" {
        check_img(base_url, attrs, &mut urllinks.img_links);
    }
}
fn check_img(base_url: &Url, attrs: &RefCell<Vec<Attribute>>, linkurls: &mut Vec<String>) {
    for attr in attrs.borrow().iter() {
        if attr.name.local.to_string() == "src" {
            create_full_url(base_url, attr, linkurls);
            break;
        }
    }
}
fn check_a(base_url: &Url, attrs: &RefCell<Vec<Attribute>>, linkurls: &mut Vec<String>) {
    for attr in attrs.borrow().iter() {
        if attr.name.local.to_string() == "href" {
            create_full_url(base_url, attr, linkurls);
            break;
        }
    }
}
fn create_full_url(base_url: &Url, attr: &Attribute, linkurls: &mut Vec<String>) {
    let path = &attr.value.to_string();
    match Url::parse(path) {
        Ok(url) => {
            linkurls.push(url.to_string());
        }
        Err(UrlParseError::RelativeUrlWithoutBase) => {
            let url = base_url.join(path).unwrap();
            linkurls.push(url.to_string());
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
#[allow(unused_variables)]
fn create_rootnode(
    url: String,
    crntdepth: usize,
    opts: &options::dl_options::Options,
) -> eyre::Result<()> {
    let root = node::element::Link::new(url.to_string());
    let mut rootlinks = node::element::Links::new(root);

    let response = reqwest::blocking::get(url).unwrap();
    let base_url = response.url().clone();
    let docstr = response.text().unwrap();
    let parser = parse_document(RcDom::default(), ParseOpts::default());
    let dom = parser.one(docstr.as_str());

    check_link(&dom, base_url, &mut rootlinks, &opts);

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
    dom: &RcDom,
    base_url: Url,
    rootlinks: &mut node::element::Links,
    opts: &options::dl_options::Options,
) {
    let mut urllinks = node::element::Urllist::new();
    walk(&base_url, 0, &dom.document, &mut urllinks);

    urllinks.a_links.sort();
    urllinks.a_links.dedup();
    urllinks.a_links.iter().for_each(|x| {
        // println!("{}", &x);
        rootlinks.add_link(x.to_string(), opts.samehost);
    });

    urllinks.img_links.sort();
    urllinks.img_links.dedup();
    urllinks.img_links.iter().for_each(|x| {
        // println!("{}", &x);
    });
}
#[allow(unused_variables)]
fn main() {
    let url = "https://qiita.com/kxkx5150";
    let opts = options::dl_options::Options::new(2, true);
    println!("\n--- start ---");
    let _ = create_rootnode(url.to_string(), 0, &opts);
    println!("---  end  ---");
}
