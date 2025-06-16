pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut marked: Vec<bool> = vec![true; (upper_bound + 1) as usize];

    for p in 2..=(upper_bound as usize) {
        for n in ((p * p)..=(upper_bound as usize)).step_by(p) {
            marked[n] = false;
        }
    }

    marked.iter()
          .enumerate()
          .filter(|&(i, v)| i > 1 && *v)
          .map(|(i, _)| i as u64)
          .collect()
}