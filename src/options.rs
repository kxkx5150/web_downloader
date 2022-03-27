pub mod dl_options {
    pub struct Options {
        pub depth: usize,
        pub samehost: bool,
    }
    impl Options {
        pub fn new(depth: usize, samehost: bool) -> Self {
            Self {
                depth,
                samehost,
            }
        }
    }
}

