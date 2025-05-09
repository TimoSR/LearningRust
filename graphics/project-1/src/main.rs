use project_1::run;

fn main() {
    //run();

    const ADD_TWO: fn (a: i32) -> i32 = |a: i32| -> i32 {
        return a + 2;
    };

    let add_three = |a: i32| {
        return a + 3;
    };

    println!("{0}",ADD_TWO(2));
    println!("{0}",add_three(2));
    println!("{0}",ADD_TWO(6));
}