// rustc 2.rs -o 2.out && ./2.out

use num_bigint::BigUint;
use num_bigint::ToBigUint;

use num_traits::identities::Zero;

fn fibonnacis(max_index: usize) -> Vec<BigUint> {
    let mut fibs: Vec<BigUint> = Vec::with_capacity(max_index);

    if max_index == 0 {
        return fibs;
    }

    fibs.push(1_u32.to_biguint().unwrap());
    if max_index == 1 {
        return fibs;
    }

    fibs.push(2_u32.to_biguint().unwrap());
    if max_index == 2 {
        return fibs;
    }

    let missing = max_index - 2;
    for _ in 0..missing {
        let prev = &fibs[fibs.len() - 1];
        let prev_prev = &fibs[fibs.len() - 2];
        fibs.push(prev + prev_prev);
    }

    return fibs;
}

fn fibonnaci_even_sum(up_to: usize) -> BigUint {
    let fibs = fibonnacis(up_to);

    let mut total = 0_u32.to_biguint().unwrap();
    for fib in fibs.iter() {
        if fib % 2_u32 == BigUint::zero() {
            total += fib;
        }
    }

    return total;
}

#[allow(dead_code)]
pub(crate) fn call() {
    println!("fibonnaci(10): {:?}", fibonnacis(10));
    // println!("fibonnaci(92): {:?}", fibonnaci(92));
    // println!("fibonnaci(93): {:?}", fibonnaci(93));
    println!("fibonnaci_even_sum(1000): {}", fibonnaci_even_sum(1000));
}
