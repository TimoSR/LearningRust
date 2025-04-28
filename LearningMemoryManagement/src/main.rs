mod stuff;
use crate::stuff::article::test::{Article, Point, Summary, Calculations};

fn main() {

    // Sometimes we may want just simple structs as data holders
    // But in most cases we want to farce  

    // Intended to fail, as we want to force going through the constructor

    // let article0 = Article {
    //     headline: String::from("hello"),
    //     location: String::from("lol"),
    // };

    fn get_str<'a>(s: *const String) -> &'a str {
        unsafe { &*s }
    }

    let value = 5.to_string();
    let str_value = get_str(&value);

    println!("\n");
    println!("{} \n", value);
    println!("{} \n", str_value);

    let boxed_article = Article::new_boxed("My own new!", "Lol!", "stuff");

    let article2 = Article::new("Testing", "ups", "stuff");

    let point1_stack = Point::new(2, 5);
    let point2_stack = Point::new(5, 18);

    // I find that ::new_boxed is a clearner approach
    let point1_heap = Point::new_boxed(10, 20);
    let point2_heap = Box::new(Point::new(3, 12));

    println!("Point1_Stack Adr: {:p} \nPoint2_Stack Adr: {:p} \nPoint1_Heap Adr: {:p} \nPoint2_Heap Adr: {:p}\n", &point1_stack, &point2_stack, &point1_heap, &point2_heap);
    println!("{:?} \n", point1_stack.distance_to(&point1_heap));

    println!("{:#?}", boxed_article);
    println!("SUM: {:#?} \n", boxed_article.summarize());

    println!("{:#?}", article2);
    println!("SUM: {:#?} \n", article2.summarize());

    println!("{:#?}", boxed_article);
    println!("SUM: {:#?} \n", boxed_article.summarize());
}
