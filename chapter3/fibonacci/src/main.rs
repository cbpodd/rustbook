fn main() {
    for i in 0..150 {
        let nth_fib = nth_fibonacci(i);
        println!("{i}th fibonacci: {nth_fib}");
    }
}

fn nth_fibonacci(n: u32) -> u128 {
    if n < 2 {
        return n.into();
    }

    let mut first_number = 0;
    let mut second_number = 1;

    for _ in 2..n {
        let new_number = first_number + second_number;
        first_number = second_number;
        second_number = new_number;
    }

    second_number
}
