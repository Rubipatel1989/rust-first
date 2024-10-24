fn main() {
    println!("Hello, world!");
    first_fun();
    passing_single_parameter(34);
    mutiple_parameters(10, 12);
    ex();
}
fn first_fun() {
    println!("Manoj Kumar");
}

fn passing_single_parameter(var: i32) {
    println!("This is my age: {}", var);
}
fn mutiple_parameters(x: i32, y: i32) {
    let c = x + y;
    println!("Sum is : {}", c);
}

fn ex() {
    let y = {
        let x = 10;
        x + 1 // No semicolon here, so this value is returned by the block
    };
    println!("Variable value Y will be: {}", y);
}
