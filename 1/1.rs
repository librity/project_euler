// def sum_of_multiples_of_3_and_5(up_to)
//   (1...up_to).select { |n| (n % 3).zero? || (n % 5).zero? }.sum
// end

fn sum_of_multiples_of_3_and_5(up_to: i32) -> i32 {
    return 0;
}

fn main() {
    println!(
        "sum_of_multiples_of_3_and_5(10): {}",
        sum_of_multiples_of_3_and_5(10)
    );
    println!(
        "sum_of_multiples_of_3_and_5(1000): {}",
        sum_of_multiples_of_3_and_5(1000)
    );
}
