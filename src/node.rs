pub mod element {
    // use url::ParseError as UrlParseError;
    use url::Url;

    pub struct Urllist {
        pub a_links: Vec<String>,
        pub img_links: Vec<String>,
        pub css_links: Vec<String>,
        pub js_links: Vec<String>,
    }
    impl Urllist {
        pub fn new() -> Self {
            Self {
                a_links: vec![],
                img_links: vec![],
                css_links: vec![],
                js_links: vec![],
            }
        }
    }

    #[allow(dead_code)]
    #[derive(Debug)]
    pub struct Link {
        url: String,
        urlstruct: Url,
    }
    impl Link {
        pub fn new(url: String) -> Self {
            let issue_list_url = Url::parse(&url).unwrap();
            Self {
                url,
                urlstruct: issue_list_url,
            }
        }
    }

    #[derive(Debug)]
    pub struct Links {
        node: Link,
        links: Vec<Link>,
        curent_idx: isize,
    }
    impl Links {
        pub fn new(node: Link) -> Self {
            Self {
                node,
                links: vec![],
                curent_idx: -1,
            }
        }
        pub fn add_link(&mut self, url: String, samehost: bool) {
            if self.check_link(&url, samehost) {
                let link = Link::new(url.to_string());
                self.links.push(link);
            }
        }
        fn check_link(&self, url: &String, samehost: bool) -> bool {
            if self.check_duplicate(url) {
                return false;
            }

            if url.starts_with("http:") || url.starts_with("https:") || !url.contains(":") {
                let prnthost = &self.node.urlstruct.host_str().unwrap().to_string();
                let urlhost = Url::parse(&url).unwrap().host_str().unwrap().to_string();

                if samehost {
                    if urlhost.starts_with(prnthost) {
                        true
                    } else {
                        false
                    }
                } else {
                    true
                }
            } else {
                false
            }
        }
        fn check_duplicate(&self, url: &String) -> bool {
            if url == "" {}
            false
        }
        pub fn len(&self) -> usize {
            self.links.len()
        }
        pub fn set_idx(&mut self, idx: isize) -> bool {
            if idx < 0 {
                self.curent_idx = -1;
                true
            } else if idx < self.len() as isize {
                self.curent_idx = idx;
                true
            } else {
                false
            }
        }
        pub fn inc(&mut self) -> bool {
            if self.set_idx(self.curent_idx + 1) {
                true
            } else {
                false
            }
        }
        pub fn curent_url(&self) -> String {
            if self.curent_idx == -1 {
                self.node.url.clone()
            } else {
                let link = &self.links[self.curent_idx as usize];
                link.url.clone()
            }
        }
        pub fn print_links(&self) {
            println!("\n--- print links ---");
            self.links.iter().for_each(|x| {
                println!("{}", &x.url);
            });
        }
    }
}
