pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
  if upper_bound < 2 {
    return vec![];
  }
  let mut sieve = vec![true; (upper_bound + 1) as usize];
  sieve[0] = false;
  sieve[1] = false;

  {
    let mut i = 2;
    while i * i <= upper_bound {
      if sieve[i as usize] {
        let mut j = 2;
        while i * j <= upper_bound {
          sieve[(i * j) as usize] = false;
          j = j + 1;
        }
      }
      i = i + 1;
    }
  }

  sieve
    .iter()
    .enumerate()
    .fold(Vec::new(), |mut result, (i, is_prime)| {
      if *is_prime {
        result.push(i as u64)
      }
      result
    })
}
