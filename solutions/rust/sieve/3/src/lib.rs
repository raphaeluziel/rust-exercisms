pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut marked = vec![true; (upper_bound + 1) as usize];

    let mut p = 2u64;
    let mut n = p * p;

    while p <= upper_bound {
        while n <= upper_bound {
            marked[n as usize] = false;
            n += p;
        }
        p += 1;
        n = p * p;
    }

    marked.iter()
    .enumerate()
          .filter(|&(i, v)| i > 1 && *v)
          .map(|(i, _)| i as u64)
          .collect()
}