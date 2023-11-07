// rustc 2.rs -o 2.out && ./2.out

use num_bigint::BigUint;
use num_bigint::ToBigUint;
use num_traits::One;
use num_traits::Zero;

#[allow(dead_code)]
fn bui_0() -> BigUint {
    0.to_biguint().unwrap()
}

fn bui_1() -> BigUint {
    1.to_biguint().unwrap()
}

fn bui_2() -> BigUint {
    2.to_biguint().unwrap()
}

fn fibonnaci_up_to_index(max_index: usize) -> Vec<BigUint> {
    let mut fibs: Vec<BigUint> = Vec::with_capacity(max_index);

    if max_index == 0 {
        return fibs;
    }

    fibs.push(bui_1());
    if max_index == 1 {
        return fibs;
    }

    fibs.push(bui_2());
    if max_index == 2 {
        return fibs;
    }

    let missing = max_index - 2;
    for _ in 0..missing {
        let prev = &fibs[fibs.len() - 1];
        let prev_prev = &fibs[fibs.len() - 2];
        let next = prev + prev_prev;

        fibs.push(next);
    }

    fibs
}

fn fibonnaci_up_to_value(max_fib: BigUint) -> Vec<BigUint> {
    let mut fibs: Vec<BigUint> = Vec::new();

    if max_fib == Zero::zero() {
        return fibs;
    }

    fibs.push(bui_1());
    if max_fib == One::one() {
        return fibs;
    }

    let two = bui_2();
    if max_fib == two {
        fibs.push(two);
        return fibs;
    }

    fibs.push(two);
    loop {
        let prev = &fibs[fibs.len() - 1];
        let prev_prev = &fibs[fibs.len() - 2];
        let next = prev + prev_prev;

        if next > max_fib {
            break;
        }

        fibs.push(prev + prev_prev);
    }

    fibs
}

fn fibonnaci_even_sum(max_fib: BigUint) -> BigUint {
    let fibs = fibonnaci_up_to_value(max_fib);

    let mut total = 0_u32.to_biguint().unwrap();
    for fib in fibs.iter() {
        if fib % 2_u32 == Zero::zero() {
            total += fib;
        }
    }

    total
}

#[allow(dead_code)]
pub(crate) fn call() {
    println!("fibonnaci_up_to_index(10): {:?}", fibonnaci_up_to_index(10));
    // println!("fibonnaci(92): {:?}", fibonnaci(92));
    // println!("fibonnaci(93): {:?}", fibonnaci(93));

    println!(
        "fibonnaci_up_to_value(10): {:?}",
        fibonnaci_up_to_value(10.to_biguint().unwrap())
    );

    println!(
        "fibonnaci_even_sum(1000): {}",
        fibonnaci_even_sum(4000000.to_biguint().unwrap())
    );
}
