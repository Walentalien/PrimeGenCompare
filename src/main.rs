use num_bigint;
use std::time::Instant;



fn main() {
    // try using the `num_bigint` crate here
    use num_bigint::BigUint;
    use num_traits::One;

    // Calculate large fibonacci numbers.
    fn fib(n: usize) -> BigUint {
        let n:BigUint = BigUint::from(n);
        let mut f0 = BigUint::ZERO;
        let mut f1 = BigUint::one();
        let mut i = BigUint::ZERO;
        let mut every100loop = Instant::now();
        let start_time = Instant::now();
        while i < n {

            let f2 = f0 + &f1;
            f0 = f1;
            f1 = f2;
            if i.clone() % BigUint::from(100_000u32) == BigUint::ZERO{
                every100loop = Instant::now();
                println!("Overall: {:?}", Instant::now()-start_time);
                println!("{}'th loop: {:?}", i.clone() / BigUint::from(100_000u32),  Instant::now()-every100loop);

            }
            i += BigUint::one();

        }
        f0
    }

    // This is a very large number.
    println!("fib(1000) = {}", fib(10_000_000));
}
