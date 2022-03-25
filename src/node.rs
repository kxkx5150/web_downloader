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
        current: Link,
        links: Vec<Link>,
    }
    impl Links {
        pub fn new(current: Link) -> Self {
            Self {
                current: current,
                links: vec![],
            }
        }
        pub fn add_link(&mut self, url: String) {
            if url.starts_with("http") {
                let link = Link::new(url.to_string());
                self.links.push(link);
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
