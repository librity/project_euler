// rustc 2.rs -o 2.out && ./2.out

fn fibonnaci(nth: i32) -> i32 {
    if nth < 2 {
        return 1;
    }

    return fibonnaci(nth - 1) + fibonnaci(nth - 2);
}

fn fibonnaci_even_sum(up_to: i32) -> i32 {
    let mut total = 0;

    for i in 1..up_to {
        print!(".");

        let fib = fibonnaci(i);
        if fib % 2 == 0 {
            total += fib;
        }
    }

    return total;
}

fn main() {
    println!("fibonnaci(10): {}", fibonnaci(10));
    println!("fibonnaci_even_sum(1000): {}", fibonnaci_even_sum(1000));
}
