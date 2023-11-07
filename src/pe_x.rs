fn function(max_fib: u32) -> u32 {
    max_fib
}

#[allow(dead_code)]
pub(crate) fn call() {
    println!("function(10): {:?}", function(10));
}
