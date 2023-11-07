fn calc_primes(up_to: u128) -> Vec<u128> {
    let mut primes: Vec<u128> = Vec::new();

    if up_to <= 1 {
        return primes;
    }

    for i in 2..=up_to {
        let mut is_prime = true;

        for prime in &primes {
            if i % prime == 0 {
                is_prime = false;
                break;
            }
        }

        if is_prime {
            primes.push(i);
        }
    }

    primes
}

fn is_prime(numb: u128) -> bool {
    if numb <= 1 {
        return false;
    }

    let primes = calc_primes(numb - 1);
    for prime in primes {
        if numb % prime == 0 {
            return false;
        }
    }

    true
}

fn prime_factors(numb: u128) -> Vec<u128> {
    let mut factors: Vec<u128> = Vec::new();

    let primes = calc_primes(numb);
    for prime in primes {
        if numb % prime == 0 {
            factors.push(prime);
        }
    }

    factors
}

fn largest_prime_factors(numb: u128) -> u128 {
    let factors: Vec<u128> = prime_factors(numb);

    *factors.last().unwrap()
}

#[allow(dead_code)]
pub(crate) fn call() {
    println!("calc_primes(14): {:?}", calc_primes(34));

    for i in 0..14 {
        println!("is_prime({}): {:?}", i, is_prime(i));
    }

    println!("prime_factors(10): {:?}", prime_factors(10));
    println!("prime_factors(13195): {:?}", prime_factors(13195));
    println!(
        "prime_factors(600851475143): {:?}",
        prime_factors(600851475143)
    );

    // println!("largest_prime_factors(10): {:?}", largest_prime_factors(10));
    // println!("largest_prime_factors(13195): {:?}", largest_prime_factors(13195));
    // println!(
    //     "largest_prime_factors(600851475143): {:?}",
    //     largest_prime_factors(600851475143)
    // );
}
