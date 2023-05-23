fn main() {
    let x = 5;
    println!("The value of x is {x}");

    let x = x + 1;
    println!("The value of x is {x}");

    {
        let x = x * 3;
        println!("The value of x in the inner scope is {x}");
    }

    println!("The value of x is {x}");

    let spaces = "   ";
    let spaces = spaces.len();

    let mut spaces = "  ";
    spaces = spaces.len();
}