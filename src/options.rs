pub mod dl_options {
    use url::Url;

    pub struct Options {
        pub rooturl: Url,
        pub depth: usize,
        pub samehost: bool,
        pub dlfolder: String,
    }
    impl Options {
        pub fn new(url: String, depth: usize, samehost: bool, dlfolder: String) -> Self {
            let rooturl = Url::parse(&url).unwrap();
            Self {
                rooturl,
                depth,
                samehost,
                dlfolder,
            }
        }
    }
}
