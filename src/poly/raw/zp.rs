//! Modular arithmetic in Zp (commonly referred as GF(p)): each element x is in the range
//! `0 <= x < p < 2^N`, where an N-bit unsigned integer stores the value.

use std::fmt;

use num_integer::Integer;
use num_traits::Unsigned;

use poly::raw::overflowing::Overflowing;

/// Unsigned integer.
pub trait UnsignedInteger: Integer + Unsigned + Overflowing + Clone + fmt::Display {}

impl<T> UnsignedInteger for T
where
    T: Integer + Unsigned + Overflowing + Clone + fmt::Display,
{
}

/// Computes `x + y` in Zp.
#[inline]
pub fn add<T: UnsignedInteger>(x: T, y: T, p: T) -> T {
    debug_assert!(x >= T::zero());
    debug_assert!(y >= T::zero());
    debug_assert!(p > T::zero());
    debug_assert!(x < p);
    debug_assert!(y < p);
    let (z, b) = T::overflowing_add(x, y);
    if b {
        T::overflowing_sub(z, p).0
    } else if z >= p {
        z - p
    } else {
        z
    }
}

/// Computes `x - y` in Zp.
#[inline]
pub fn sub<T: UnsignedInteger>(x: T, y: T, p: T) -> T {
    debug_assert!(x >= T::zero());
    debug_assert!(y >= T::zero());
    debug_assert!(p > T::zero());
    debug_assert!(x < p);
    debug_assert!(y < p);
    let (z, b) = T::overflowing_sub(x, y);
    if b {
        T::overflowing_add(z, p).0
    } else {
        z
    }
}

/// Computes `x * y` in Zp.
#[inline]
pub fn mul<T: UnsignedInteger>(x: T, y: T, p: T) -> T {
    debug_assert!(x >= T::zero());
    debug_assert!(y >= T::zero());
    debug_assert!(p > T::zero());
    debug_assert!(x < p);
    debug_assert!(y < p);
    if x.is_zero() {
        T::zero()
    } else {
        // by Schrage's method
        let q = p.clone() / x.clone();
        let r = p.clone() % x.clone();
        let a = x * (y.clone() % q.clone());
        let b = if r <= q {
            r * (y / q)
        } else {
            mul(r, y / q, p.clone())
        };
        sub(a, b, p)
    }
}

/// Computes `-x` in Zp.
#[inline]
pub fn neg<T: UnsignedInteger>(x: T, p: T) -> T {
    debug_assert!(x >= T::zero());
    debug_assert!(p > T::zero());
    debug_assert!(x < p);
    if x.is_zero() {
        T::zero()
    } else {
        p - x
    }
}

/// Computes `1/x` in Zp.
#[inline]
pub fn inv<T: UnsignedInteger>(x: T, p: T) -> T {
    debug_assert!(x > T::zero());
    debug_assert!(p > T::zero());
    debug_assert!(x < p);
    // by the extended Euclidean algorithm: a x + b p = gcd(x, p) = 1 or a x = 1 (mod p)
    // taken from https://www.di-mgt.com.au/euclidean.html#code-modinv, which is based on Knuth
    // vol. 2, Algorithm X
    let mut u1 = T::one();
    let mut u3 = x.clone();
    let mut v1 = T::zero();
    let mut v3 = p.clone();
    let mut even_iter: bool = true;
    while !v3.is_zero() {
        let q = u3.clone() / v3.clone();
        let t3 = u3 % v3.clone();
        let t1 = u1 + q * v1.clone();
        u1 = v1;
        v1 = t1;
        u3 = v3;
        v3 = t3;
        even_iter = !even_iter;
    }
    assert!(u3 == T::one(), "{} is not invertible in Z_{}", x, p);
    if even_iter {
        u1
    } else {
        p - u1
    }
}

/// Computes `x^n` in Zp.
pub fn pow<T: UnsignedInteger>(x: T, n: u32, p: T) -> T {
    debug_assert!(x >= T::zero());
    debug_assert!(p > T::zero());
    debug_assert!(x < p);
    if x.is_zero() {
        if n == 0 {
            return T::one();
        } else {
            return T::zero();
        }
    }
    if n == 0 {
        return T::one();
    }
    if n == 1 {
        return x;
    }
    if n == 2 {
        let y = x.clone();
        return mul(x, y, p);
    }
    // TODO: to be improved
    let mut r = x.clone();
    for _ in 1..n {
        r = mul(r, x.clone(), p.clone());
    }
    r
}

#[test]
fn test_add() {
    fn check_add(x: u8, y: u8, p: u8) {
        let x64 = x as u64;
        let y64 = y as u64;
        let p64 = p as u64;
        let z64 = add(x, y, p) as u64;
        assert_eq!(z64, (x64 + y64) % p64);
    }

    check_add(100, 200, 251);
    check_add(100, 151, 251);
    check_add(100, 100, 251);
}

#[test]
fn test_sub() {
    fn check_sub(x: u8, y: u8, p: u8) {
        let x64 = x as u64;
        let y64 = y as u64;
        let p64 = p as u64;
        let z64 = sub(x, y, p) as u64;
        assert_eq!(z64, (p64 + x64 - y64) % p64);
    }

    check_sub(100, 200, 251);
    check_sub(200, 100, 251);
}

#[test]
fn test_mul() {
    fn check_mul(x: u8, y: u8, p: u8) {
        let x64 = x as u64;
        let y64 = y as u64;
        let p64 = p as u64;
        let z64 = mul(x, y, p) as u64;
        assert_eq!(z64, x64 * y64 % p64);
    }

    check_mul(100, 200, 251);
    check_mul(11, 23, 251);
    check_mul(10, 20, 251);

    check_mul(250, 250, 251);
    check_mul(250, 2, 251);
    check_mul(2, 250, 251);
    check_mul(16, 16, 251);
    check_mul(128, 2, 251);
    check_mul(2, 128, 251);

    check_mul(0, 0, 251);
    check_mul(0, 1, 251);
    check_mul(0, 250, 251);
    check_mul(1, 0, 251);
    check_mul(250, 0, 251);
}

#[test]
fn test_neg() {
    fn check_neg(x: u8, p: u8) {
        let z = neg(x, p);
        assert_eq!(add(x, z, p), 0);
    }

    check_neg(0, 251);
    check_neg(1, 251);
    check_neg(2, 251);
    check_neg(10, 251);
    check_neg(16, 251);
    check_neg(31, 251);
    check_neg(100, 251);
    check_neg(200, 251);
    check_neg(249, 251);
    check_neg(250, 251);
}

#[test]
fn test_inv() {
    fn check_inv(x: u8, p: u8) {
        let z = inv(x, p);
        assert_eq!(mul(x, z, p), 1);
    }

    check_inv(1, 251);
    check_inv(2, 251);
    check_inv(10, 251);
    check_inv(16, 251);
    check_inv(31, 251);
    check_inv(100, 251);
    check_inv(200, 251);
    check_inv(249, 251);
    check_inv(250, 251);
}

#[test]
fn test_pow() {
    fn check_pow(x: u8, n: u8, p: u8) {
        let x64 = x as u64;
        let n32 = n as u32;
        let p64 = p as u64;
        let z64 = pow(x, n as u32, p) as u64;
        assert_eq!(z64, x64.pow(n32) % p64);
    }

    check_pow(0, 0, 251);
    check_pow(1, 0, 251);
    check_pow(2, 0, 251);
    check_pow(0, 1, 251);
    check_pow(1, 1, 251);
    check_pow(2, 1, 251);
    check_pow(0, 2, 251);
    check_pow(1, 2, 251);
    check_pow(2, 2, 251);

    check_pow(3, 3, 241);
    check_pow(3, 4, 241);
    check_pow(3, 5, 241);
    check_pow(3, 6, 241);

    check_pow(3, 3, 251);
    check_pow(3, 4, 251);
    check_pow(3, 5, 251);
    check_pow(3, 6, 251);

    check_pow(10, 3, 251);
    check_pow(10, 4, 251);
    check_pow(10, 5, 251);
    check_pow(10, 6, 251);

    check_pow(101, 3, 251);
    check_pow(101, 4, 251);
    check_pow(101, 5, 251);
    check_pow(101, 6, 251);
}
