

/*
Algorithm:
1. Create list of states(True/False) of numbers from 2
up to some given limit
2. Starting from 2 up to ceil(sqrt(limit))  : Mark every multiple of number marked true as false(not prime)
(Why ceil(sqrt(limit))? because limit could be divisible by at most (ceil(sqrt(limit))
and if its not, then it's not divisible by any number from range higher than that ( because if it was we would
find the second divisor which has to be smaller than ceil(sqrt(limit)) but we didnt
*/
use num_bigint::BigUint;

pub fn sieve_of_eratosthenes(limit: BigUint) -> Vec<BigUint> {
    if limit < 2 {
     return Vec::new(); // No primes if limit is less than 2
    }

    // Initialize the sieve with true values up to `limit`

    let mut curr_num: u64 = 2;

    let mut square  = curr_num * curr_num


    // Iterate up to the square root of `limit`
    while curr_num * curr_num <= limit{


        if sieve[curr_num as usize] {
            // Mark all multiples of `curr_num` as non-prime
            let mut to_check_out = square;
            while to_check_out <= limit {
                sieve[to_check_out as usize] = false;

                // Safely increment `to_check_out` by `curr_num` to get the next multiple
                to_check_out = match to_check_out.checked_add(curr_num) {
                    Some(new_num) => new_num,
                    None => break, // Break if an overflow occurs
                };
            }
        }

        curr_num += 1;
    }

    // Collect indices marked as `true` in `sieve`, which represent prime numbers
    sieve.iter()
        .enumerate()
        .filter_map(|(num, &is_prime)| if is_prime && num >= 2 { Some(num as u32) } else { None })
        .collect()
}

