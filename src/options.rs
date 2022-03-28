pub mod dl_options {
    pub struct Options {
        pub depth: usize,
        pub samehost: bool,
        pub dlfolder: String,
    }
    impl Options {
        pub fn new(depth: usize, samehost: bool, dlfolder:String) -> Self {
            Self {
                depth,
                samehost,
                dlfolder,
            }
        }
    }
}

