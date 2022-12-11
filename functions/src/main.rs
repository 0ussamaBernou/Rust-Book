fn main() {
    println!("Hello, world!");

    another_function();
    let mut _y /*this is a statement */= 5 /*and thats an expression */;
    _y = {
        // Every scope created with curly braces is an expression
        let x = 5;
        x + 1 // if this line was to end with a ';' it would turn into a statement thus returning nothing
    };
    println!("{}", _y); // a macro is also an expression

    println!("{}", five());

    println!("{}", plus_one(_y));
}

fn another_function() {
    println!("Another Function!");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
