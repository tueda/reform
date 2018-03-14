use std::cmp::Ordering;
use std::fmt;
use std::mem;
use std::ops::{Add, Mul, Neg, Sub};

use num_traits::{One, Zero};

use poly::exponent::Exponent;
use poly::ring::Ring;

/// Multivariate polynomial with a degree sparse and variable dense representation.
#[derive(Clone)]
pub struct MultivariatePolynomial<R: Ring, E: Exponent> {
    // Data format: the i-th monomial is stored as coefficients[i] and
    // exponents[i * nvars .. (i + 1) * nvars]. Keep coefficients.len() == nterms and
    // exponents.len() == nterms * nvars. Terms are always expanded and sorted by the exponents via
    // cmp_exponents().
    coefficients: Vec<R>,
    exponents: Vec<E>,
    nterms: usize,
    nvars: usize,
}

impl<R: Ring, E: Exponent> MultivariatePolynomial<R, E> {
    /// Constructs a zero polynomial.
    #[inline]
    pub fn new() -> Self {
        Self {
            coefficients: Vec::new(),
            exponents: Vec::new(),
            nterms: 0,
            nvars: 0,
        }
    }

    /// Constructs a zero polynomial with the given number of variables.
    #[inline]
    pub fn with_nvars(nvars: usize) -> Self {
        Self {
            coefficients: Vec::new(),
            exponents: Vec::new(),
            nterms: 0,
            nvars: nvars,
        }
    }

    /// Constructs a constant polynomial with the given number of variables.
    #[inline]
    pub fn from_constant_with_nvars(constant: R, nvars: usize) -> Self {
        if constant.is_zero() {
            return Self::with_nvars(nvars);
        }
        Self {
            coefficients: vec![constant],
            exponents: vec![E::zero(); nvars],
            nterms: 1,
            nvars: nvars,
        }
    }

    /// Constructs a polynomial with a single term.
    #[inline]
    pub fn from_monomial(coefficient: R, exponents: Vec<E>) -> Self {
        if coefficient.is_zero() {
            return Self::with_nvars(exponents.len());
        }
        Self {
            coefficients: vec![coefficient],
            nvars: exponents.len(),
            exponents: exponents,
            nterms: 1,
        }
    }

    /// Returns the number of terms in the polynomial.
    #[inline]
    pub fn nterms(&self) -> usize {
        return self.nterms;
    }

    /// Returns the number of variables in the polynomial.
    #[inline]
    pub fn nvars(&self) -> usize {
        return self.nvars;
    }

    /// Returns true if the polynomial is constant.
    #[inline]
    pub fn is_constant(&self) -> bool {
        if self.is_zero() {
            return true;
        }
        if self.nterms >= 2 {
            return false;
        }
        debug_assert!(!self.coefficients.first().unwrap().is_zero());
        return self.exponents.iter().all(|e| e.is_zero());
    }

    /// Returns the slice for the exponents of the specified monomial.
    #[inline]
    fn exponents(&self, index: usize) -> &[E] {
        &self.exponents[index * self.nvars..(index + 1) * self.nvars]
    }

    /// Returns the mutable slice for the exponents of the specified monomial.
    #[inline]
    fn exponents_mut(&mut self, index: usize) -> &mut [E] {
        &mut self.exponents[index * self.nvars..(index + 1) * self.nvars]
    }

    /// Compares exponent vectors of two monomials.
    #[inline]
    fn cmp_exponents(a: &[E], b: &[E]) -> Ordering {
        debug_assert!(a.len() == b.len());
        // TODO: Introduce other term orders.
        a.cmp(b)
    }

    /// Appends a monomial to the polynomial.
    pub fn append_monomial(&mut self, coefficient: R, mut exponents: Vec<E>) {
        if coefficient.is_zero() {
            return;
        }
        if self.nvars != exponents.len() {
            panic!("nvars mismatched");
        }
        // Linear search to find the insert-point.
        // TODO: Binary search.
        for i in 0..self.nterms {
            let c;
            {
                let a = self.exponents(i);
                let b = &exponents[..];
                c = Self::cmp_exponents(a, b);
            }
            if c == Ordering::Equal {
                // Add the two coefficients.
                let mut new_coeff =
                    mem::replace(&mut self.coefficients[i], R::zero()).add(coefficient);
                if new_coeff.is_zero() {
                    // The coefficient becomes zero. Remove this monomial.
                    self.coefficients.remove(i);
                    let i = i * self.nvars;
                    self.exponents.splice(i..i + self.nvars, Vec::new());
                    self.nterms -= 1;
                } else {
                    // Set the new coefficient.
                    mem::swap(&mut self.coefficients[i], &mut new_coeff);
                }
                return;
            } else if c == Ordering::Greater {
                // Insert the monomial at this point.
                self.coefficients.insert(i, coefficient);
                let i = i * self.nvars;
                self.exponents.splice(i..i, exponents);
                self.nterms += 1;
                return;
            }
        }
        // Push the monomial at the last.
        self.coefficients.push(coefficient);
        self.exponents.append(&mut exponents);
        self.nterms += 1;
    }
}

impl<R: Ring, E: Exponent> Default for MultivariatePolynomial<R, E> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

/// View object for a term in a multivariate polynomial.
#[derive(Clone, Debug)]
pub struct MultivariateMonomialView<'a, R: 'a, E: 'a> {
    pub coefficient: &'a R,
    pub exponents: &'a [E],
}

/// Iterator over terms in a multivariate polynomial.
pub struct MultivariateMonomialViewIterator<'a, R: 'a + Ring, E: 'a + Exponent> {
    poly: &'a MultivariatePolynomial<R, E>,
    index: usize,
}

impl<'a, R: Ring, E: Exponent> Iterator for MultivariateMonomialViewIterator<'a, R, E> {
    type Item = MultivariateMonomialView<'a, R, E>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.poly.nterms {
            None
        } else {
            let view = MultivariateMonomialView {
                coefficient: &self.poly.coefficients[self.index],
                exponents: self.poly.exponents(self.index),
            };
            self.index += 1;
            Some(view)
        }
    }
}

impl<'a, R: Ring, E: Exponent> IntoIterator for &'a MultivariatePolynomial<R, E> {
    type Item = MultivariateMonomialView<'a, R, E>;
    type IntoIter = MultivariateMonomialViewIterator<'a, R, E>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            poly: self,
            index: 0,
        }
    }
}

impl<R: Ring + fmt::Debug, E: Exponent + fmt::Debug> fmt::Debug for MultivariatePolynomial<R, E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_zero() {
            return write!(f, "[]");
        }
        let mut first = true;
        write!(f, "[ ")?;
        for monomial in self {
            if first {
                first = false;
            } else {
                write!(f, ", ")?;
            }
            write!(
                f,
                "{{ {:?}, {:?} }}",
                monomial.coefficient, monomial.exponents
            )?;
        }
        write!(f, " ]")
    }
}

impl<R: Ring + fmt::Display, E: Exponent + One + fmt::Display> fmt::Display
    for MultivariatePolynomial<R, E>
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut is_first_term = true;
        for monomial in self {
            if monomial.coefficient.is_zero() {
                continue;
            }
            let mut is_first_factor = true;
            if monomial.coefficient.eq(&R::one()) {
                if !is_first_term {
                    write!(f, "+")?;
                }
            } else if monomial.coefficient.eq(&R::one().neg()) {
                write!(f, "-")?;
            } else {
                if is_first_term {
                    write!(f, "({})", monomial.coefficient)?;
                } else {
                    write!(f, "+({})", monomial.coefficient)?;
                }
                is_first_factor = false;
            }
            is_first_term = false;
            for (i, e) in monomial.exponents.into_iter().enumerate() {
                if e.is_zero() {
                    continue;
                }
                if is_first_factor {
                    is_first_factor = false;
                } else {
                    write!(f, "*")?;
                }
                write!(f, "x{}", i)?;
                if e.ne(&E::one()) {
                    write!(f, "^{}", e)?;
                }
            }
            if is_first_factor {
                write!(f, "1")?;
            }
        }
        if is_first_term {
            write!(f, "0")?;
        }
        Ok(())
    }
}

impl<R: Ring + PartialEq, E: Exponent> PartialEq for MultivariatePolynomial<R, E> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        if self.nvars != other.nvars {
            if self.is_zero() && other.is_zero() {
                // Both are 0.
                return true;
            }
            if self.is_zero() || other.is_zero() {
                // One of them is 0.
                return false;
            }
            panic!("nvars mismatched");
        }
        if self.nterms != other.nterms {
            return false;
        }
        self.exponents.eq(&other.exponents) && self.coefficients.eq(&other.coefficients)
    }
}

impl<R: Ring + Eq, E: Exponent> Eq for MultivariatePolynomial<R, E> {}

impl<R: Ring, E: Exponent> Zero for MultivariatePolynomial<R, E> {
    #[inline]
    fn zero() -> Self {
        Self::new()
    }

    #[inline]
    fn is_zero(&self) -> bool {
        self.nterms == 0
    }
}

impl<R: Ring, E: Exponent> Add for MultivariatePolynomial<R, E> {
    type Output = Self;

    fn add(mut self, mut other: Self) -> Self::Output {
        if self.is_zero() {
            return other;
        }
        if other.is_zero() {
            return self;
        }
        if self.nvars != other.nvars {
            panic!("nvars mismatched");
        }

        // Merge the two polynomials, which are assumed to be already sorted.

        let mut new_coefficients = Vec::new();
        let mut new_exponents: Vec<E> = Vec::new();
        let mut new_nterms = 0;
        let mut i = 0;
        let mut j = 0;

        while i < self.nterms && j < other.nterms {
            let c = Self::cmp_exponents(self.exponents(i), other.exponents(j));
            match c {
                Ordering::Less => {
                    Self::add_push(
                        &mut new_coefficients,
                        &mut new_exponents,
                        &mut new_nterms,
                        &mut self,
                        i,
                    );
                    i += 1;
                }
                Ordering::Greater => {
                    Self::add_push(
                        &mut new_coefficients,
                        &mut new_exponents,
                        &mut new_nterms,
                        &mut other,
                        j,
                    );
                    j += 1;
                }
                Ordering::Equal => {
                    let c1 = mem::replace(&mut self.coefficients[i], R::zero());
                    let c2 = mem::replace(&mut other.coefficients[j], R::zero());
                    let mut new_c = c1.add(c2);
                    if !new_c.is_zero() {
                        mem::swap(&mut self.coefficients[i], &mut new_c);
                        Self::add_push(
                            &mut new_coefficients,
                            &mut new_exponents,
                            &mut new_nterms,
                            &mut self,
                            i,
                        );
                    }
                    i += 1;
                    j += 1;
                }
            }
        }

        while i < self.nterms {
            Self::add_push(
                &mut new_coefficients,
                &mut new_exponents,
                &mut new_nterms,
                &mut self,
                i,
            );
            i += 1;
        }

        while j < other.nterms {
            Self::add_push(
                &mut new_coefficients,
                &mut new_exponents,
                &mut new_nterms,
                &mut other,
                j,
            );
            j += 1;
        }

        Self {
            coefficients: new_coefficients,
            exponents: new_exponents,
            nterms: new_nterms,
            nvars: self.nvars,
        }
    }
}

impl<R: Ring, E: Exponent> MultivariatePolynomial<R, E> {
    #[inline(always)]
    fn add_push(
        new_coefficients: &mut Vec<R>,
        new_exponents: &mut Vec<E>,
        new_nterms: &mut usize,
        source: &mut Self,
        i: usize,
    ) {
        new_coefficients.push(mem::replace(&mut source.coefficients[i], R::zero()));
        new_exponents.reserve(source.nvars);
        for e in source.exponents_mut(i) {
            new_exponents.push(mem::replace(e, E::zero()));
        }
        *new_nterms += 1;
    }
}

impl<R: Ring, E: Exponent> Sub for MultivariatePolynomial<R, E> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        self.add(other.neg())
    }
}

impl<R: Ring, E: Exponent> Neg for MultivariatePolynomial<R, E> {
    type Output = Self;
    fn neg(mut self) -> Self::Output {
        // Negate coefficients of all terms.
        for c in &mut self.coefficients {
            let mut new_c = mem::replace(c, R::zero()).neg();
            mem::swap(c, &mut new_c);
        }
        self
    }
}

impl<R: Ring, E: Exponent> Mul for MultivariatePolynomial<R, E> {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        if self.is_zero() {
            return Self::with_nvars(other.nvars);
        }
        if other.is_zero() {
            return Self::with_nvars(self.nvars);
        }
        if self.nvars != other.nvars {
            panic!("nvars mismatched");
        }
        // TODO: this is a quick implementation. To be improved.
        let mut new_poly = Self::with_nvars(self.nvars);
        for m in &other {
            let p = self.clone().mul_monomial(m.coefficient, m.exponents);
            new_poly = new_poly.add(p);
        }
        new_poly
    }
}

impl<R: Ring, E: Exponent> MultivariatePolynomial<R, E> {
    #[inline]
    fn mul_monomial(mut self, coefficient: &R, exponents: &[E]) -> Self {
        debug_assert!(self.nvars == exponents.len());
        debug_assert!(self.nterms > 0);
        debug_assert!(!coefficient.is_zero());
        for c in &mut self.coefficients {
            let mut new_c = mem::replace(c, R::zero()).mul(coefficient.clone());
            mem::swap(c, &mut new_c);
        }
        for i in 0..self.nterms {
            let ee = self.exponents_mut(i);
            for (e1, e2) in ee.iter_mut().zip(exponents) {
                *e1 = e1.checked_add(e2).expect("overflow in adding exponents");
            }
        }
        self
    }
}

impl<R: Ring, E: Exponent> MultivariatePolynomial<R, E> {
    #[inline]
    fn divexact_monomial(
        dividend_coefficient: &R,
        dividend_exponents: &[E],
        divisor_coefficient: &R,
        divisor_exponents: &[E],
        result_coefficient: &mut R,
        result_exponents: &mut [E],
    ) -> bool {
        debug_assert!(dividend_exponents.len() == divisor_exponents.len());
        debug_assert!(dividend_exponents.len() == result_exponents.len());
        if dividend_exponents
            .iter()
            .zip(divisor_exponents.iter())
            .any(|(a, b)| a.cmp(b) == Ordering::Less)
        {
            return false;
        }
        if !dividend_coefficient
            .clone()
            .rem(divisor_coefficient.clone())
            .is_zero()
        {
            return false;
        }
        *result_coefficient = divisor_coefficient.clone().div(divisor_coefficient.clone());
        for (i, e) in result_exponents.iter_mut().enumerate() {
            *e = dividend_exponents[i]
                .clone()
                .sub(divisor_exponents[i].clone());
        }
        return true;
    }
}