fn main() {
    println!("Hello, world!");
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {}", count);
    while_conditional();
    liftoff();
}

fn while_conditional() {
    let mut num = 3;

    while num != 0 {
        println!("{num}");

        num -= 1
    }
    println!("liftoff!");
}

fn liftoff() {
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LiftOff!");
}
