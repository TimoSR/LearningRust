mod article;

//use article::timosr::Article;
use article::timosr::{Article, Summary};

fn main() {
    // Intended to fail, as we want to force going through the constructor

    // let article0 = Article {
    //     headline: String::from("hello"),
    //     location: String::from("lol"),
    // };

    let article1 = Article::heap("My own new!", "Lol!");

    let article2 = Article::stack("Testing", "ups");

    //println!("{:#?}", article0);

    println!("{:#?}", article1);
    println!("{:#?} \n", article1.summarize());
    println!("{:#?}", article2);
    println!("{:#?} \n", article2.summarize());

    // pub trait Summary {
    //     fn summarize(&self) -> String;
    // }

    // #[derive(Debug)]
    // pub struct Article<'a> {
    //     pub headline: &'a str,
    //     pub location: &'a str,
    // }

    // impl<'a> Article<'a> {
    //     pub fn new(headline: &'a str, location: &'a str) -> Self {
    //         Article { headline, location }
    //     }
    // }

    // impl<'a> Summary for Article<'a> {
    //     fn summarize(&self) -> String {
    //         format!("{}, by {}", self.headline, self.location)
    //     }
    // }

    // let article = Article {
    //     headline: "Hello World",
    //     location: "Copenhagen",
    // };

    // let article2: Box<Article<'_>> = Box::new(Article {
    //     headline: "I love boxing!",
    //     location: "Lol!",
    // });

    // let article3: Article<'_> = Article::new("My own new!", "Lol!");

    // let article4 = Box::new(Article::new("Testing", "ups"));

    // pub trait Summary {
    //     fn summarize(&self) -> String;
    // }

    // #[derive(Debug)]
    // pub struct Article {
    //     pub headline: String,
    //     pub location: String,
    // }

    // impl Article {
    //     pub fn new(headline: String, location: String) -> Self {
    //         Article { headline, location }
    //     }
    // }

    // impl Summary for Article {
    //     fn summarize(&self) -> String {
    //         format!("{}, by {}", self.headline, self.location)
    //     }
    // }

    // let article = Article {
    //     headline: String::from("Hello World"),
    //     location: String::from("Copenhagen"),
    // };

    // let article2 = Box::new(Article {
    //     headline: String::from("I love boxing!"),
    //     location: String::from("Lol!"),
    // });

    // let article3 = Article::new(String::from("My own new!"), String::from("Lol!"));

    // let article4 = Box::new(Article::new(String::from("Testing"), String::from("ups")));
}
