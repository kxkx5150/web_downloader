// use std::cell::RefCell;
// use std::rc::Rc;
// use html5ever::{Attribute, LocalName, QualName};
use html5ever::driver::ParseOpts;
// use html5ever::{local_name, ns, namespace_url};
use html5ever::{parse_document, parse_fragment};
use html5ever::rcdom::{Handle, Node, NodeData, RcDom};
// use html5ever::serialize;
// use html5ever::serialize::SerializeOpts;
use html5ever::tendril::{TendrilSink, StrTendril};
#[allow(unused_variables)]
fn walk(indent: usize, handle: &Handle) {
    let node = handle;
    // FIXME: don't allocate
    // print!("{}", repeat(" ").take(indent).collect::<String>());
    match node.data {
        NodeData::Document => {
            // println!("#Document")
        },

        NodeData::Doctype {
            ref name,
            ref public_id,
            ref system_id,
        } => {
            // println!("<!DOCTYPE {} \"{}\" \"{}\">", name, public_id, system_id),
        },
        NodeData::Text { ref contents } => {
            // println!("#text: {}", contents.borrow().escape_default())
        },

        NodeData::Comment { ref contents } => {
            // println!("<!-- {} -->", contents.escape_default()),
        },
        NodeData::Element {
            ref name,
            ref attrs,
            ..
        } => {
            print!("<{}", name.local);
            for attr in attrs.borrow().iter() {
                print!(" {}=\"{}\"", attr.name.local, attr.value);
            }
            println!(">");
        },

        NodeData::ProcessingInstruction { .. } => unreachable!(),
    }

    for child in node.children.borrow().iter() {
        walk(indent + 4, child);
    }
}

fn main() {
    let url = "http://www.brokenthorn.com/Resources/OSDevIndex.html";
    let response = reqwest::blocking::get(url).unwrap();
    let base_url = response.url().clone();
    let docstr = response.text().unwrap();
    let parser = parse_document(RcDom::default(), ParseOpts::default());
    let dom = parser.one(docstr.as_str());
    println!("\n--- start ---");
    walk(0, &dom.document);
    println!("---  end  ---");

}