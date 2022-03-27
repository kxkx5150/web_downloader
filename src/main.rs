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
use std::cell::RefCell;
use std::rc::Rc;
use url::ParseError as UrlParseError;
use url::Url;


#[allow(unused_variables)]
fn walk(base_url: &Url, indent: usize, node: &Handle) {
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
            check_tag(base_url, name.local.to_string(), &attrs);
            // print!("<{}", name.local);
            // for attr in attrs.borrow().iter() {
            //     print!(" {}=\"{}\"", attr.name.local, attr.value);
            // }
            // println!(">");
        }
        NodeData::ProcessingInstruction { .. } => unreachable!(),
    }

    for child in node.children.borrow().iter() {
        walk(base_url, indent + 4, child);
    }
}
#[allow(unused_variables)]
fn check_tag(base_url: &Url, nodestr: String, attrs: &RefCell<Vec<Attribute>>) {
    if nodestr == "a" {
        for attr in attrs.borrow().iter() {
            if attr.name.local.to_string() == "href" {
                let path = &attr.value.to_string();
                match Url::parse(path) {
                    Ok(url) => {
                        println!("{}", url);
                    }
                    Err(UrlParseError::RelativeUrlWithoutBase) => {
                        let url = base_url.join(path).unwrap();
                        println!("{}", url);
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }
                break;
            }
        }
    }
}

#[allow(unused_variables)]
fn main() {
    let url = "http://www.brokenthorn.com/Resources/OSDevIndex.html";
    let response = reqwest::blocking::get(url).unwrap();
    let base_url = response.url().clone();
    let docstr = response.text().unwrap();
    let parser = parse_document(RcDom::default(), ParseOpts::default());
    let dom = parser.one(docstr.as_str());

    println!("\n--- start ---");
    walk(&base_url, 0, &dom.document);
    println!("---  end  ---");
}
