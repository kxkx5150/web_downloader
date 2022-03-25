#[allow(dead_code)]

pub mod element {

    #[derive(Debug)]
    pub struct Link {
        url: String,
        domain: String,
        path: String,
        dir: String,
        filename: String,
    }
    impl Link {
        pub fn new(url: String) -> Self {
            Self {
                url,
                domain: String::from(""),
                path: String::from(""),
                dir: String::from(""),
                filename: String::from(""),
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
        pub fn add_link(&mut self, url: String) {
            if url.starts_with("http") {
                let link = Link::new(url.to_string());
                self.links.push(link);
            }
        }
        pub fn len(&self) -> usize {
            self.links.len()
        }
        pub fn curent_url(&self) -> String {
            // let &url = self.links
            "".to_string()
        }
        pub fn print_links(&self) {
            println!("\n--- print links ---");
            self.links.iter().for_each(|x| {
                println!("{}", &x.url);
            });
        }
    }
}
