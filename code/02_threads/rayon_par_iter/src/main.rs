use std::time::Instant;

use rayon::prelude::*;

fn is_prime(n: u32) -> bool {
    (2 ..= n/2).into_par_iter().all(|i| n % i != 0 )
 }

fn main() {
    let numbers: Vec<u64> = (0 .. 1_000_000).collect();
    let sum = numbers.par_iter().sum::<u64>();
    println!("Sum: {sum}");
    println!();

    // Print primes below 1,000
    let now = Instant::now();
    let numbers: Vec<u64> = (2 .. 1_000_000).collect();
    let mut primes: Vec<&u64> = numbers.par_iter().filter(|&n| is_prime(*n as u32)).collect();
    primes.sort();
    let elapsed = now.elapsed();
    //println!("{primes:?}");
    println!("It took {} ms to find {} primes and sort them", elapsed.as_millis(), primes.len());

    // Do the same again, but this time Rayon will do the sorting
    let now = Instant::now();
    let numbers: Vec<u64> = (2 .. 1_000_000).collect();
    let mut primes: Vec<&u64> = numbers.par_iter().filter(|&n| is_prime(*n as u32)).collect();
    primes.par_sort_unstable();
    let elapsed = now.elapsed();
    //println!("{primes:?}");
    println!("It took {} ms to find {} primes, including a parallel sort", elapsed.as_millis(), primes.len());
}
