pub fn factors(n: u64) -> Vec<u64> {
    let mut res = vec![];

    if n < 2 {
        return res;
    }
    let mut n = n;
    let mut start = 2;

    'begin: loop {
        for i in start..(n / 2 + 1) {
            if n % i == 0 {
                res.push(i);
                n /= i;
                start = i;
                continue 'begin;
            }
        }
        res.push(n);
        break 'begin;
    }
    res
}
