pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
  if upper_bound < 2 {
    return vec![];
  }
  let mut sieve = vec![true; (upper_bound + 1) as usize];
  sieve[0] = false;
  sieve[1] = false;

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

  let mut result = Vec::new();
  for (number, is_prime) in sieve.into_iter().enumerate() {
    if is_prime {
      result.push(number as u64);
    }
  }
  result
}
