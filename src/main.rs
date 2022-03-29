use html5ever::driver::ParseOpts;
use html5ever::rcdom::{Handle, NodeData, RcDom};
use html5ever::serialize;
use html5ever::serialize::SerializeOpts;
use html5ever::tendril::{StrTendril, TendrilSink};
use html5ever::{namespace_url, ns};
use html5ever::{parse_document, Attribute, LocalName, QualName};
use std::cell::RefCell;
use std::fs;
use std::io::Write;
use std::path::Path;
use url::{ParseError as UrlParseError, Position, Url};
mod node;
mod options;
pub use crate::node::element;
pub use crate::options::dl_options;

#[allow(unused_variables)]
fn walk(
    base_url: &Url,
    indent: usize,
    node: &Handle,
    urllinks: &mut node::element::Urllist,
    opts: &options::dl_options::Options,
) {
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
            check_tag(base_url, name.local.to_string(), &attrs, urllinks, opts);
        }
        NodeData::ProcessingInstruction { .. } => unreachable!(),
    }

    for child in node.children.borrow().iter() {
        walk(base_url, indent + 4, child, urllinks, opts);
    }
}
#[allow(unused_variables)]
fn check_tag(
    base_url: &Url,
    nodestr: String,
    attrs: &RefCell<Vec<Attribute>>,
    urllinks: &mut node::element::Urllist,
    opts: &options::dl_options::Options,
) {
    if nodestr == "a" {
        let dlpath = check_a(base_url, attrs, &mut urllinks.a_links);
        // convert_url_to_localpath(base_url, attrs, &dlpath, "href".to_string(), opts);
    } else if nodestr == "img" {
        let dlpath = check_img(base_url, attrs, &mut urllinks.img_links);
        convert_url_to_localpath(base_url, attrs, &dlpath, "src".to_string(), opts);
    } else if nodestr == "link" {
        let dlpath = check_css(base_url, attrs, &mut urllinks.css_links);
        // convert_url_to_localpath(base_url, attrs, &dlpath, "href".to_string(), opts);
    } else if nodestr == "script" {
        let dlpath = check_script(base_url, attrs, &mut urllinks.js_links);
        // convert_url_to_localpath(base_url, attrs, &dlpath, "src".to_string(), opts);
    }
}
#[allow(unused_variables)]
fn convert_url_to_localpath(
    base_url: &Url,
    attrs: &RefCell<Vec<Attribute>>,
    fpath: &String,
    name: String,
    opts: &options::dl_options::Options,
) {
    // let (fullpath, dirpath) = create_download_path(fpath, opts);
    // let homeurl = base_url.scheme().to_string() + "://" +  base_url.host_str().unwrap();
    // let mut localpath: String = "".to_string();

    // if fpath.starts_with(&homeurl){
    //     let aaaa = opts.dlfolder.to_string() + base_url.host_str().unwrap();
    //     localpath = fullpath.replace(&aaaa.to_string(), ".");
    // }else {
    //     localpath = fullpath.replace(&opts.dlfolder.to_string(), "../");
    // }

    // if localpath.ends_with("/") {
    //     localpath = localpath + "index.html";
    // }
    // println!("\n{}",homeurl );
    // println!("{}", fpath.to_string());
    // println!("{}", fullpath.to_string());
    // println!("{}", &localpath);

    // attrs.borrow_mut().push(create_attribute(
    //     &name,
    //     &localpath,
    // ));
    // println!("edit");
}
#[allow(dead_code)]
fn create_attribute(name: &str, value: &str) -> Attribute {
    Attribute {
        name: QualName::new(None, ns!(), LocalName::from(name)),
        value: StrTendril::from(value),
    }
}
fn check_a(base_url: &Url, attrs: &RefCell<Vec<Attribute>>, linkurls: &mut Vec<String>) -> String {
    for attr in attrs.borrow().iter() {
        if attr.name.local.to_string() == "href" {
            return create_full_url(base_url, &attr.value.to_string(), linkurls).to_string();
        }
    }
    return "".to_string();
}
fn check_img(
    base_url: &Url,
    attrs: &RefCell<Vec<Attribute>>,
    linkurls: &mut Vec<String>,
) -> String {
    for attr in attrs.borrow().iter() {
        if attr.name.local.to_string() == "src" {
            return create_full_url(base_url, &attr.value.to_string(), linkurls).to_string();
        }
    }
    return "".to_string();
}
fn check_css(
    base_url: &Url,
    attrs: &RefCell<Vec<Attribute>>,
    linkurls: &mut Vec<String>,
) -> String {
    let mut dlpath = "".to_string();
    let mut cssflg = false;
    for attr in attrs.borrow().iter() {
        if attr.name.local.to_string() == "href" {
            if cssflg {
                return create_full_url(base_url, &attr.value.to_string(), linkurls).to_string();
            } else {
                dlpath = attr.value.to_string();
            }
        }

        if attr.name.local.to_string() == "rel" {
            if attr.value.to_string() == "stylesheet" {
                if dlpath != "" {
                    return create_full_url(base_url, &attr.value.to_string(), linkurls)
                        .to_string();
                } else {
                    cssflg = true;
                }
            }
        }
    }
    return "".to_string();
}
fn check_script(
    base_url: &Url,
    attrs: &RefCell<Vec<Attribute>>,
    linkurls: &mut Vec<String>,
) -> String {
    for attr in attrs.borrow().iter() {
        if attr.name.local.to_string() == "src" {
            return create_full_url(base_url, &attr.value.to_string(), linkurls).to_string();
        }
    }
    return "".to_string();
}
fn create_full_url(base_url: &Url, path: &String, linkurls: &mut Vec<String>) -> String {
    if path == "" {
        return "".to_string();
    }
    match Url::parse(path) {
        Ok(url) => {
            let urlstr = url.to_string();
            linkurls.push(urlstr.clone());
            urlstr.to_string()
        }
        Err(UrlParseError::RelativeUrlWithoutBase) => {
            let urlstr = base_url.join(path).unwrap().to_string();
            linkurls.push(urlstr.clone());
            urlstr.to_string()
        }
        Err(e) => {
            println!("Error: {}", e);
            "".to_string()
        }
    }
}
fn check_link(
    dom: &RcDom,
    base_url: Url,
    rootlinks: &mut node::element::Links,
    opts: &options::dl_options::Options,
) {
    println!("\n--- start ---");
    let mut urllinks = node::element::Urllist::new();
    walk(&base_url, 0, &dom.document, &mut urllinks, opts);

    download_page(dom, &base_url, opts);

    urllinks.a_links.sort();
    urllinks.a_links.dedup();
    urllinks.a_links.iter().for_each(|x| {
        rootlinks.add_link(x.to_string(), opts.samehost);
    });
    {
        iter_download_list(&mut urllinks.img_links, &opts);
    }
    {
        iter_download_list(&mut urllinks.css_links, &opts);
    }
    {
        iter_download_list(&mut urllinks.js_links, &opts);
    }
}
#[allow(dead_code)]
fn download_page(dom: &RcDom, base_url: &Url, opts: &options::dl_options::Options) {
    let mut htmlpath: String = base_url.to_string();
    if htmlpath.ends_with("/") {
        htmlpath = base_url.join("index.html").unwrap().to_string();
    }
    let (fullpath, dirpath) = create_download_path(&htmlpath, opts);

    println!("{}", base_url.to_string());
    println!("{}", fullpath.to_string());
    println!("{}", dirpath.to_string());

    let _ = fs::create_dir_all(dirpath);
    let mut bytes = vec![];
    serialize(&mut bytes, &dom.document, SerializeOpts::default()).unwrap();
    let mut file = std::fs::File::create(fullpath).unwrap();
    let _ = file.write_all(&bytes);
}
fn iter_download_list(linklist: &mut Vec<String>, opts: &options::dl_options::Options) {
    linklist.sort();
    linklist.dedup();
    linklist.iter().for_each(|x| {
        download_file(&x, opts);
    });
}

#[allow(unused_variables)]
fn download_file(url: &String, opts: &options::dl_options::Options) {
    let (fullpath, dirpath) = create_download_path(url, opts);
    println!("\n{}", &dirpath);
    println!("{}", &fullpath);

    let _ = fs::create_dir_all(dirpath);
    let mut file = std::fs::File::create(fullpath).unwrap();
    reqwest::blocking::get(url)
        .unwrap()
        .copy_to(&mut file)
        .unwrap();
}
fn create_download_path(url: &String, opts: &options::dl_options::Options) -> (String, String) {
    let urlobj = Url::parse(&url).unwrap();
    let host = urlobj.host_str().unwrap();
    let urlpath = &urlobj[Position::BeforePath..];
    let filepath = host.to_string() + urlpath;
    let path = Path::new(&filepath);

    let basepath = &opts.dlfolder;
    let parent = path.parent().unwrap().to_str().unwrap();
    let filename = &urlobj.path();
    let fullpath = basepath.to_string() + host + filename;
    let dirpath = basepath.to_string() + parent + "/";

    return (fullpath.to_string(), dirpath.to_string());
}
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

    println!("\n");
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
fn main() {
    let url = "https://www.as-web.jp/f1";
    let opts = options::dl_options::Options::new(
        url.to_string(),
        2,
        true,
        "./web_downloader_rust/".to_string(),
    );
    let _ = create_rootnode(url.to_string(), 0, &opts);
}
