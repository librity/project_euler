// rustc 1.rs -o 1.out && ./1.out

fn sum_of_multiples_of_3_and_5(up_to: i32) -> i32 {
    let mut total: i32 = 0;

    for i in 1..up_to {
        if i % 3 == 0 || i % 5 == 0 {
            total += i;
        }
    }

    return total;
}

#[allow(dead_code)]
pub(crate) fn call() {
    println!(
        "sum_of_multiples_of_3_and_5(10): {}",
        sum_of_multiples_of_3_and_5(10)
    );
    println!(
        "sum_of_multiples_of_3_and_5(1000): {}",
        sum_of_multiples_of_3_and_5(1000)
    );
}
