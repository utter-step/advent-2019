#[inline(always)]
pub(super) fn mult128(x: i64, y: i64, modulo: i64) -> i64 {
    ((x as i128 * y as i128).rem_euclid(modulo as i128)) as i64
}

pub fn bin_pow(mut x: i64, mut pow_to: usize, modulo: i64) -> i64 {
    let mut res = 1;
    while pow_to > 0 {
        if pow_to & 1 == 1 {
            res = mult128(res, x, modulo);
        }
        x = mult128(x, x, modulo);
        pow_to >>= 1;
    }
    res
}

struct GcdSolution {
    gcd: i64,
    x: i64,
    #[allow(dead_code)]
    y: i64,
}

/// Compute Extended Euclidean algorithm for GCD.
///
/// Code is stolen [from wiki][1]:
///
/// ```pseudo
/// function extended_gcd(a, b)
///     (old_r, r) := (a, b)
///     (old_s, s) := (1, 0)
///     (old_t, t) := (0, 1)
///
///     while r ≠ 0 do
///         quotient := old_r div r
///         (old_r, r) := (r, old_r − quotient × r)
///         (old_s, s) := (s, old_s − quotient × s)
///         (old_t, t) := (t, old_t − quotient × t)
///
///     output "Bézout coefficients:", (old_s, old_t)
///     output "greatest common divisor:", old_r
///     output "quotients by the gcd:", (t, s)
/// ```
///
/// [1]: https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm#Pseudocode
#[allow(clippy::many_single_char_names)]
fn gcd_extended(a: i64, b: i64) -> GcdSolution {
    let mut old_r = a;
    let mut r = b;

    let mut old_s = 1;
    let mut s = 0;

    let mut old_t = 0;
    let mut t = 1;

    while r != 0 {
        let quot = old_r / r;

        let r_temp = r;
        r = old_r - quot * r;
        old_r = r_temp;

        let s_temp = s;
        s = old_s - quot * s;
        old_s = s_temp;

        let t_temp = t;
        t = old_t - quot * t;
        old_t = t_temp;
    }

    GcdSolution {
        x: old_s,
        y: old_t,
        gcd: old_r,
    }
}

pub(super) fn inverse_mod_root(n: i64, modulo: i64) -> i64 {
    let gcd_extended = gcd_extended(n, modulo);
    let gcd = gcd_extended.gcd;
    assert_eq!(
        gcd.abs(),
        1,
        "numbers must be coprime for this method to work!"
    );

    let x = gcd_extended.x * gcd;
    (x.rem_euclid(modulo) + modulo).rem_euclid(modulo)
}

pub(super) fn sum_of_pows(x: i64, pow_to: usize, modulo: i64) -> i64 {
    assert_ne!(x.abs(), 1, "number must be greater than zero");

    let root = inverse_mod_root(x - 1, modulo);

    mult128(bin_pow(x, pow_to + 1, modulo) - 1, root, modulo)
}

#[cfg(test)]
mod tests {
    use super::*;

    use quickcheck::TestResult;

    const MODULO: i64 = 119315717514047;

    fn sum_of_pows_naive(x: i64, pow_to: usize, modulo: i64) -> i64 {
        let mut cur = 1;
        let mut s: i64 = 0;

        for _ in 0..=pow_to {
            s = (s + cur).rem_euclid(modulo);
            cur = mult128(cur, x, modulo);
        }

        s
    }

    #[test]
    fn test_simple_cases() {
        assert_eq!(sum_of_pows(2, 1, MODULO), 3);

        assert_eq!(sum_of_pows(2, 2, MODULO), 7);

        assert_eq!(sum_of_pows(2, 3, MODULO), 15);

        assert_eq!(sum_of_pows(-2, 1, MODULO), 119315717514046);
    }

    #[quickcheck]
    fn sum_of_pows_compared_to_naive(x: i64, pow_to: usize) -> TestResult {
        if pow_to == 0 || pow_to > 1000 || x.abs() <= 1 {
            return TestResult::discard();
        }

        assert_eq!(
            sum_of_pows(x, pow_to, MODULO),
            sum_of_pows_naive(x, pow_to, MODULO),
        );

        TestResult::passed()
    }
}
