pub mod timosr {

    pub trait Summary {
        fn summarize(&self) -> String;
    }

    #[derive(Debug)]
    pub struct Article {
        headline: String,
        location: String,
    }

    impl Article {
        pub fn heap(headline: &str, location: &str) -> Box<Self> {
            return Box::new(Article {
                headline: String::from(headline),
                location: String::from(location),
            });
        }

        pub fn stack(headline: &str, location: &str) -> Self {
            return Article {
                headline: String::from(headline),
                location: String::from(location),
            };
        }
    }

    impl Summary for Article {
        fn summarize(&self) -> String {
            format!("{}, by {}", self.headline, self.location)
        }
    }
}
