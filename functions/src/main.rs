fn five() -> i32 {
    5
}

fn main() {
    let x = five();
    let y = {
        let x = 3;
        x + 1
    };

    another_function(x, y);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is {}", y);
}
