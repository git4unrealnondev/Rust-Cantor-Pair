///
/// Gets a unique value based on two inputs
///
fn cantor_pair(n: &usize, m: &usize) -> usize {
    (n + m) * (n + m + 1) / 2 + m
}

///
/// Gets the unique inputs from a cantor number
///
fn cantor_unpair(z: &usize) -> (usize, usize) {
    let w64 = (8 * z + 1) as f64;
    let w64two = ((w64.sqrt() - 1.0) / 2.0).floor() as usize;
    let t = (w64two * w64two + w64two) / 2;
    let m = z - t;
    let n = w64two - m;
    return (n, m);
}

#[cfg(test)]
mod tests {
    use super::*;

    ///
    /// Tests some values that I picked out
    ///
    #[test]
    fn ensure_equal_values() {
        let test_values = [(0, 0), (0, 1), (1, 0), (5, 5), (10000, 10000)];
        for (a, b) in test_values {
            let cantor = cantor_pair(&a, &b);
            let (aout, bout) = cantor_unpair(&cantor);
            dbg!(cantor, aout, bout, a, b);
            assert_eq!((a, b), (aout, bout));
        }
    }
}
