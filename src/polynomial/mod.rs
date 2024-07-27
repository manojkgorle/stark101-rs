use chrono::Local;

use crate::field::{Field, FieldElement};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
// How should we be interpolating polynomials?
// -> lagrange interpolation
// -> iNTT inverse Number Theoretic Transformation
// How should we be evaluating polynomials?
// -> Normal evaluation
// -> NTT Number Theoretic Transformation

/// a_0 + a_1 * x + a_2 * x^2 + a_3 * x^3 + ... + a_n * x^n
#[derive(Debug, Clone)]
pub struct Polynomial {
    pub coefficients: Vec<FieldElement>,
    pub evaluation_form: Vec<(FieldElement, FieldElement)>,
}

impl Polynomial {
    pub fn new_from_coefficients(coefficients: Vec<FieldElement>) -> Self {
        Polynomial {
            coefficients,
            evaluation_form: Vec::new(),
        }
    }

    pub fn new_from_evaluation(evaluation_form: Vec<(FieldElement, FieldElement)>) -> Self {
        Polynomial {
            coefficients: Vec::new(),
            evaluation_form,
        }
    }

    pub fn evaluate(&self, x: FieldElement) -> FieldElement {
        let mut result = FieldElement::new(0, Field::new(x.modulus()));

        for i in 0..self.coefficients.len() {
            result += self.coefficients[i] * x.pow(i as u64);
        }
        result
    }

    pub fn degree(&self) -> usize {
        self.coefficients.len() - 1
    }

    pub fn scalar_mul(&self, scalar: FieldElement) -> Self {
        let mut result = Vec::new();

        for i in 0..self.coefficients.len() {
            result.push(self.coefficients[i] * scalar);
        }
        // self.coefficients = result.clone();
        Polynomial::new_from_coefficients(result)
    }

    pub fn is_all_zeros(&self) -> bool {
        for i in 0..self.coefficients.len() {
            if self.coefficients[i].0 != 0 {
                return false;
            }
        }
        true
    }

    pub fn scalar_div(&self, scalar: FieldElement) -> Self {
        let mut result = Vec::new();

        for i in 0..self.coefficients.len() {
            result.push(self.coefficients[i] / scalar);
        }
        // self.coefficients = result.clone();
        Polynomial::new_from_coefficients(result)
    }

    pub fn q_div(self, poly2: Self) -> (Self, Self) {
        let mut q = Vec::new();
        let field = Field::new(self.coefficients[0].modulus());
        let n = self.coefficients.len();
        let m = poly2.coefficients.len();
        if n < m {
            return (
                Polynomial::new_from_coefficients(vec![FieldElement::new(0, field)]),
                self,
            );
        }
        let mut poly1_coeff = self.clone().coefficients;
        let mut poly2_coeff = poly2.clone().coefficients;
        poly1_coeff.reverse();
        poly2_coeff.reverse();
        for i in 0..n - m + 1 {
            let mut other_coeff = poly2_coeff.clone();
            other_coeff.append(&mut vec![FieldElement::new(0, field); n - m - i]);
            let q_temp = poly1_coeff[0] / other_coeff[0];
            let other_poly = Polynomial::new_from_coefficients(other_coeff).scalar_mul(q_temp);
            poly1_coeff = (Polynomial::new_from_coefficients(poly1_coeff) - other_poly.clone())
                .coefficients[1..]
                .to_vec();
            q.push(q_temp);
        }
        q.reverse();
        poly1_coeff.reverse();
        let poly1 = Polynomial::new_from_coefficients(poly1_coeff);
        if poly1.is_all_zeros() {
            return (
                Polynomial::new_from_coefficients(q),
                Polynomial::new_from_coefficients(vec![FieldElement::new(0, field)]),
            );
        } else {
            return (Polynomial::new_from_coefficients(q), poly1);
        }
    }

    pub fn compose(mut self, mono: FieldElement) -> Self {
        for i in 0..self.coefficients.len() {
            self.coefficients[i] *= mono.pow(i as u64);
        }
        return self;
    }
}

impl Add for Polynomial {
    type Output = Polynomial;

    fn add(self, other: Polynomial) -> Polynomial {
        let mut result = Vec::new();
        let mut i = 0;

        while i < self.coefficients.len() && i < other.coefficients.len() {
            result.push(self.coefficients[i] + other.coefficients[i]);
            i += 1;
        }

        while i < self.coefficients.len() {
            result.push(self.coefficients[i]);
            i += 1;
        }

        while i < other.coefficients.len() {
            result.push(other.coefficients[i]);
            i += 1;
        }

        Polynomial::new_from_coefficients(result)
    }
}

impl AddAssign for Polynomial {
    fn add_assign(&mut self, other: Polynomial) {
        let mut result = Vec::new();
        let mut i = 0;

        while i < self.coefficients.len() && i < other.coefficients.len() {
            result.push(self.coefficients[i] + other.coefficients[i]);
            i += 1;
        }

        while i < self.coefficients.len() {
            result.push(self.coefficients[i]);
            i += 1;
        }

        while i < other.coefficients.len() {
            result.push(other.coefficients[i]);
            i += 1;
        }

        self.coefficients = result;
    }
}

impl Sub for Polynomial {
    type Output = Polynomial;

    fn sub(self, other: Polynomial) -> Polynomial {
        let mut result = Vec::new();
        let mut i = 0;

        while i < self.coefficients.len() && i < other.coefficients.len() {
            result.push(self.coefficients[i] - other.coefficients[i]);
            i += 1;
        }

        while i < self.coefficients.len() {
            result.push(self.coefficients[i]);
            i += 1;
        }

        while i < other.coefficients.len() {
            result.push(-other.coefficients[i]);
            i += 1;
        }

        Polynomial::new_from_coefficients(result)
    }
}

impl SubAssign for Polynomial {
    fn sub_assign(&mut self, other: Polynomial) {
        let mut result = Vec::new();
        let mut i = 0;

        while i < self.coefficients.len() && i < other.coefficients.len() {
            result.push(self.coefficients[i] - other.coefficients[i]);
            i += 1;
        }

        while i < self.coefficients.len() {
            result.push(self.coefficients[i]);
            i += 1;
        }

        while i < other.coefficients.len() {
            result.push(-other.coefficients[i]);
            i += 1;
        }

        self.coefficients = result;
    }
}

impl Mul for Polynomial {
    type Output = Polynomial;

    fn mul(self, other: Polynomial) -> Polynomial {
        let field = Field::new(self.coefficients[0].modulus());
        let mut result = vec![
            FieldElement::new(0, field);
            self.coefficients.len() + other.coefficients.len() - 1
        ];

        for i in 0..self.coefficients.len() {
            for j in 0..other.coefficients.len() {
                result[i + j] += self.coefficients[i] * other.coefficients[j];
            }
        }

        Polynomial::new_from_coefficients(result)
    }
}

impl MulAssign for Polynomial {
    fn mul_assign(&mut self, other: Polynomial) {
        let mut result = vec![
            FieldElement::new(0, Field::new(0));
            self.coefficients.len() + other.coefficients.len() - 1
        ];

        for i in 0..self.coefficients.len() {
            for j in 0..other.coefficients.len() {
                result[i + j] += self.coefficients[i] * other.coefficients[j];
            }
        }

        self.coefficients = result;
    }
}

impl Div for Polynomial {
    type Output = Polynomial;

    fn div(self, other: Polynomial) -> Polynomial {
        let (q, r) = self.q_div(other);
        if r.is_all_zeros() {
            q
        } else {
            panic!("Division not possible");
        }
    }
}

impl DivAssign for Polynomial {
    fn div_assign(&mut self, other: Polynomial) {
        let (q, r) = self.clone().q_div(other);
        if r.is_all_zeros() {
            self.coefficients = q.coefficients;
        } else {
            panic!("Division not possible");
        }
    }
}

pub fn gen_polynomial_from_roots(roots: Vec<FieldElement>) -> Polynomial {
    let field = Field::new(roots[0].modulus());
    let mut result = vec![FieldElement::new(1, field); 1];
    for i in 0..roots.len() {
        let mut new_result = vec![FieldElement::new(0, field); 2];
        new_result[0] = -roots[i];
        new_result[1] = FieldElement::new(1, field);
        let new_polynomial = Polynomial::new_from_coefficients(new_result);
        result = (Polynomial::new_from_coefficients(result) * new_polynomial).coefficients;
    }
    Polynomial::new_from_coefficients(result[..roots.len() + 1].to_vec())
}

pub fn gen_lagrange_polynomials(x: Vec<FieldElement>) -> Vec<Polynomial> {
    let n = x.len();
    let mut lagrange_polynomials = Vec::new();

    for i in 0..n {
        let mut denominator = Vec::new();

        let roots = &x[..i];
        log::debug!("generating polynomial from roots, i:{:?}", i);
        let numerator = gen_polynomial_from_roots([roots, &x[i + 1..]].concat());
        for j in 0..n {
            if i == j {
                continue;
            }
            denominator.push(x[i] - x[j]);
        }
        let den_sum = denominator.iter().fold(
            FieldElement::new(1, Field::new(x[i].modulus())),
            |acc, x| acc * *x,
        );
        let lagrange_polynomial = numerator.scalar_div(den_sum);
        lagrange_polynomials.push(lagrange_polynomial);
    }

    lagrange_polynomials
}

pub fn interpolate_lagrange_polynomials(x: Vec<FieldElement>, y: Vec<FieldElement>) -> Polynomial {
    let n = x.len();
    log::info!("generating lagrange polynomials");
    let start_time = Local::now();
    let lagrange_polynomials = gen_lagrange_polynomials(x.clone());
    let field = Field::new(x[0].modulus());
    let mut result = Polynomial::new_from_coefficients(vec![FieldElement::new(0, field); n]);

    for i in 0..n {
        result += lagrange_polynomials[i].scalar_mul(y[i]);
    }
    log::info!(
        "lagrange polynomials generated in {:?}µs",
        (Local::now() - start_time).num_microseconds().unwrap()
    );
    result
}

#[cfg(test)]
mod test_polynomials {
    use super::*;

    #[test]
    fn test_evaluate() {
        let field = Field::new(7);
        let coefficients = vec![FieldElement::new(1, field), FieldElement::new(2, field)];
        let polynomial = Polynomial::new_from_coefficients(coefficients);
        let x = FieldElement::new(4, field);
        let result = polynomial.evaluate(x);
        assert_eq!(result.0, 2);
    }

    #[test]
    fn test_add() {
        let field = Field::new(7);
        let coefficients1 = vec![FieldElement::new(1, field), FieldElement::new(2, field)];
        let coefficients2 = vec![FieldElement::new(3, field), FieldElement::new(4, field)];
        let polynomial1 = Polynomial::new_from_coefficients(coefficients1);
        let polynomial2 = Polynomial::new_from_coefficients(coefficients2);
        let result = polynomial1 + polynomial2;
        assert_eq!(result.coefficients[0].0, 4);
        assert_eq!(result.coefficients[1].0, 6);
    }

    #[test]
    fn test_add_poly2_larger() {
        let field = Field::new(7);
        let coefficients1 = vec![FieldElement::new(1, field), FieldElement::new(2, field)];
        let coefficients2 = vec![
            FieldElement::new(3, field),
            FieldElement::new(4, field),
            FieldElement::new(5, field),
        ];
        let polynomial1 = Polynomial::new_from_coefficients(coefficients1);
        let polynomial2 = Polynomial::new_from_coefficients(coefficients2);
        let result = polynomial1 + polynomial2;
        assert_eq!(result.coefficients[0].0, 4);
        assert_eq!(result.coefficients[1].0, 6);
        assert_eq!(result.coefficients[2].0, 5);
    }

    #[test]
    fn test_add_poly1_larger() {
        let field = Field::new(7);
        let coefficients1 = vec![
            FieldElement::new(1, field),
            FieldElement::new(4, field),
            FieldElement::new(3, field),
        ];
        let coefficients2 = vec![FieldElement::new(3, field), FieldElement::new(4, field)];
        let polynomial1 = Polynomial::new_from_coefficients(coefficients1);
        let polynomial2 = Polynomial::new_from_coefficients(coefficients2);
        let result = polynomial1 + polynomial2;
        assert_eq!(result.coefficients[0].0, 4);
        assert_eq!(result.coefficients[1].0, 1);
        assert_eq!(result.coefficients[2].0, 3);
    }

    #[test]
    fn test_sub() {
        let field = Field::new(7);
        let coefficients1 = vec![FieldElement::new(1, field), FieldElement::new(2, field)];
        let coefficients2 = vec![FieldElement::new(3, field), FieldElement::new(4, field)];
        let polynomial1 = Polynomial::new_from_coefficients(coefficients1);
        let polynomial2 = Polynomial::new_from_coefficients(coefficients2);
        let result = polynomial1 - polynomial2;
        assert_eq!(result.coefficients[0].0, 5);
        assert_eq!(result.coefficients[1].0, 5);
    }

    #[test]
    fn test_sub_poly2_larger() {
        let field = Field::new(7);
        let coefficients1 = vec![FieldElement::new(1, field), FieldElement::new(2, field)];
        let coefficients2 = vec![
            FieldElement::new(3, field),
            FieldElement::new(4, field),
            FieldElement::new(5, field),
        ];
        let polynomial1 = Polynomial::new_from_coefficients(coefficients1);
        let polynomial2 = Polynomial::new_from_coefficients(coefficients2);
        let result = polynomial1 - polynomial2;
        assert_eq!(result.coefficients[0].0, 5);
        assert_eq!(result.coefficients[1].0, 5);
        assert_eq!(result.coefficients[2].0, 2);
    }

    #[test]
    fn test_sub_poly1_larger() {
        let field = Field::new(7);
        let coefficients1 = vec![
            FieldElement::new(1, field),
            FieldElement::new(4, field),
            FieldElement::new(3, field),
        ];
        let coefficients2 = vec![FieldElement::new(3, field), FieldElement::new(4, field)];
        let polynomial1 = Polynomial::new_from_coefficients(coefficients1);
        let polynomial2 = Polynomial::new_from_coefficients(coefficients2);
        let result = polynomial1 - polynomial2;
        assert_eq!(result.coefficients[0].0, 5);
        assert_eq!(result.coefficients[1].0, 0);
        assert_eq!(result.coefficients[2].0, 3);
    }

    #[test]
    fn test_mul() {
        let field = Field::new(7);
        let coefficients1 = vec![FieldElement::new(1, field), FieldElement::new(2, field)];
        let coefficients2 = vec![FieldElement::new(3, field), FieldElement::new(4, field)];
        let polynomial1 = Polynomial::new_from_coefficients(coefficients1);
        let polynomial2 = Polynomial::new_from_coefficients(coefficients2);
        let result = polynomial1 * polynomial2;
        assert_eq!(result.coefficients[0].0, 3);
        assert_eq!(result.coefficients[1].0, 3);
        assert_eq!(result.coefficients[2].0, 1);
    }

    #[test]
    fn test_scalar_mul() {
        let field = Field::new(7);
        let coefficients = vec![FieldElement::new(1, field), FieldElement::new(2, field)];
        let polynomial = Polynomial::new_from_coefficients(coefficients);
        let scalar = FieldElement::new(3, field);
        let result = polynomial.scalar_mul(scalar);
        assert_eq!(result.coefficients[0].0, 3);
        assert_eq!(result.coefficients[1].0, 6);
    }

    #[test]
    fn test_scalar_div() {
        let field = Field::new(7);
        let coefficients = vec![FieldElement::new(1, field), FieldElement::new(2, field)];
        let polynomial = Polynomial::new_from_coefficients(coefficients);
        let scalar = FieldElement::new(4, field);
        let result = polynomial.scalar_div(scalar);
        assert_eq!(result.coefficients[0].0, 2);
        assert_eq!(result.coefficients[1].0, 4);
    }

    #[test]
    fn test_polynomial_from_roots() {
        let field = Field::new(7);
        let roots = vec![
            FieldElement::new(1, field),
            FieldElement::new(2, field),
            FieldElement::new(3, field),
        ];
        let polynomial = gen_polynomial_from_roots(roots);
        assert_eq!(polynomial.coefficients[0].0, 1);
        assert_eq!(polynomial.coefficients[1].0, 4);
        assert_eq!(polynomial.coefficients[2].0, 1);
        assert_eq!(polynomial.coefficients[3].0, 1);
    }

    #[test]
    fn test_gen_lagrange_poly() {
        let field = Field::new(7);
        let x = vec![
            FieldElement::new(2, field),
            FieldElement::new(3, field),
            FieldElement::new(5, field),
        ];
        let lagrange_polynomials = gen_lagrange_polynomials(x);
        assert_eq!(lagrange_polynomials.len(), 3);
        assert_eq!(lagrange_polynomials[0].coefficients[0].0, 5);
        assert_eq!(lagrange_polynomials[0].coefficients[1].0, 2);
        assert_eq!(lagrange_polynomials[0].coefficients[2].0, 5);
        assert_eq!(lagrange_polynomials[1].coefficients[0].0, 2);
        assert_eq!(lagrange_polynomials[1].coefficients[1].0, 0);
        assert_eq!(lagrange_polynomials[1].coefficients[2].0, 3);
        assert_eq!(lagrange_polynomials[2].coefficients[0].0, 1);
        assert_eq!(lagrange_polynomials[2].coefficients[1].0, 5);
        assert_eq!(lagrange_polynomials[2].coefficients[2].0, 6);
    }

    #[test]
    fn test_gen_lagrange_poly2() {
        let field = Field::new(7);
        let x = vec![
            FieldElement::new(2, field),
            FieldElement::new(3, field),
            FieldElement::new(5, field),
            FieldElement::new(6, field),
        ];
        let lagrange_polynomials = gen_lagrange_polynomials(x);
        assert_eq!(lagrange_polynomials.len(), 4);
        assert_eq!(lagrange_polynomials[0].coefficients[0].0, 4);
        assert_eq!(lagrange_polynomials[0].coefficients[1].0, 0);
        assert_eq!(lagrange_polynomials[0].coefficients[2].0, 0);
        assert_eq!(lagrange_polynomials[0].coefficients[3].0, 4);
        assert_eq!(lagrange_polynomials[1].coefficients[0].0, 4);
        assert_eq!(lagrange_polynomials[1].coefficients[1].0, 4);
        assert_eq!(lagrange_polynomials[1].coefficients[2].0, 6);
        assert_eq!(lagrange_polynomials[1].coefficients[3].0, 6);
        assert_eq!(lagrange_polynomials[2].coefficients[0].0, 6);
        assert_eq!(lagrange_polynomials[2].coefficients[1].0, 1);
        assert_eq!(lagrange_polynomials[2].coefficients[2].0, 3);
        assert_eq!(lagrange_polynomials[2].coefficients[3].0, 1);
        assert_eq!(lagrange_polynomials[3].coefficients[0].0, 1);
        assert_eq!(lagrange_polynomials[3].coefficients[1].0, 2);
        assert_eq!(lagrange_polynomials[3].coefficients[2].0, 5);
        assert_eq!(lagrange_polynomials[3].coefficients[3].0, 3);
    }

    #[test]
    fn test_interpolate_lagrange_polynomials() {
        let field = Field::new(7);
        let x = vec![
            FieldElement::new(2, field),
            FieldElement::new(3, field),
            FieldElement::new(5, field),
        ];
        let y = vec![
            FieldElement::new(1, field),
            FieldElement::new(2, field),
            FieldElement::new(3, field),
        ];
        let result = interpolate_lagrange_polynomials(x, y);
        assert_eq!(result.coefficients[0].0, 5);
        assert_eq!(result.coefficients[1].0, 3);
        assert_eq!(result.coefficients[2].0, 1);
    }

    #[test]
    fn test_qdiv() {
        let field = Field::new(7);
        let coefficients1 = vec![
            FieldElement::new(1, field),
            FieldElement::new(2, field),
            FieldElement::new(3, field),
            FieldElement::new(4, field),
        ];
        let coefficients2 = vec![
            FieldElement::new(1, field),
            FieldElement::new(2, field),
            FieldElement::new(3, field),
        ];
        let polynomial1 = Polynomial::new_from_coefficients(coefficients1);
        let polynomial2 = Polynomial::new_from_coefficients(coefficients2);
        let (q, r) = polynomial1.q_div(polynomial2);
        assert_eq!(q.coefficients.len(), 2);
        assert_eq!(r.coefficients.len(), 2);
        assert_eq!(q.coefficients[0].0, 4);
        assert_eq!(q.coefficients[1].0, 6);
        assert_eq!(r.coefficients[0].0, 4);
        assert_eq!(r.coefficients[1].0, 2);
    }

    #[test]
    fn test_qdiv2() {
        let field = Field::new(7);

        let coefficients1 = vec![
            FieldElement::new(2, field),
            FieldElement::new(1, field),
            FieldElement::new(3, field),
            FieldElement::new(0, field),
            FieldElement::new(6, field),
        ];

        let coefficients2 = vec![
            FieldElement::new(3, field),
            FieldElement::new(5, field),
            FieldElement::new(0, field),
            FieldElement::new(4, field),
        ];

        let polynomial1 = Polynomial::new_from_coefficients(coefficients1);
        let polynomial2 = Polynomial::new_from_coefficients(coefficients2);
        let (q, r) = polynomial1.q_div(polynomial2);

        assert_eq!(r.coefficients.len(), 3);
        assert_eq!(q.coefficients.len(), 2);
        assert_eq!(r.coefficients[0].0, 2);
        assert_eq!(r.coefficients[1].0, 0);
        assert_eq!(r.coefficients[2].0, 6);
        assert_eq!(q.coefficients[0].0, 0);
        assert_eq!(q.coefficients[1].0, 5);
    }

    #[test]
    fn test_qdiv_same_n_m() {
        let field = Field::new(7);
        let coefficients1 = vec![
            FieldElement::new(1, field),
            FieldElement::new(2, field),
            FieldElement::new(3, field),
        ];
        let coefficients2 = vec![
            FieldElement::new(2, field),
            FieldElement::new(3, field),
            FieldElement::new(4, field),
        ];

        let polynomial1 = Polynomial::new_from_coefficients(coefficients1);
        let polynomial2 = Polynomial::new_from_coefficients(coefficients2);

        let (q, r) = polynomial1.q_div(polynomial2);
        assert_eq!(q.coefficients.len(), 1);
        assert_eq!(r.coefficients.len(), 2);
        assert_eq!(q.coefficients[0].0, 6);
        assert_eq!(r.coefficients[0].0, 3);
        assert_eq!(r.coefficients[1].0, 5);
    }

    #[test]
    fn test_qdiv_m_larger_than_n() {
        let field = Field::new(7);
        let coefficients1 = vec![
            FieldElement::new(1, field),
            FieldElement::new(2, field),
            FieldElement::new(3, field),
        ];
        let coefficients2 = vec![
            FieldElement::new(2, field),
            FieldElement::new(3, field),
            FieldElement::new(4, field),
            FieldElement::new(5, field),
        ];

        let polynomial1 = Polynomial::new_from_coefficients(coefficients1);
        let polynomial2 = Polynomial::new_from_coefficients(coefficients2);

        let (q, r) = polynomial1.q_div(polynomial2);
        assert_eq!(q.coefficients.len(), 1);
        assert_eq!(r.coefficients.len(), 3);
        assert_eq!(q.coefficients[0].0, 0);
        assert_eq!(r.coefficients[0].0, 1);
        assert_eq!(r.coefficients[1].0, 2);
        assert_eq!(r.coefficients[2].0, 3);
    }

    #[test]
    fn test_qdiv_no_remainder() {
        let field = Field::new(7);

        let coefficients1 = vec![
            FieldElement::new(1, field),
            FieldElement::new(2, field),
            FieldElement::new(3, field),
        ];

        let coefficients2 = vec![
            FieldElement::new(1, field),
            FieldElement::new(2, field),
            FieldElement::new(3, field),
        ];

        let polynomial1 = Polynomial::new_from_coefficients(coefficients1);
        let polynomial2 = Polynomial::new_from_coefficients(coefficients2);

        let (q, r) = polynomial1.q_div(polynomial2);

        assert_eq!(q.coefficients.len(), 1);
        assert_eq!(r.coefficients.len(), 1);
        assert_eq!(q.coefficients[0].0, 1);
        assert_eq!(r.coefficients[0].0, 0);
    }

    #[test]
    fn test_div() {
        let field = Field::new(7);

        let coefficients1 = vec![
            FieldElement::new(1, field),
            FieldElement::new(2, field),
            FieldElement::new(3, field),
        ];

        let coefficients2 = vec![
            FieldElement::new(1, field),
            FieldElement::new(2, field),
            FieldElement::new(3, field),
        ];

        let polynomial1 = Polynomial::new_from_coefficients(coefficients1);
        let polynomial2 = Polynomial::new_from_coefficients(coefficients2);

        let result = polynomial1 / polynomial2;

        assert_eq!(result.coefficients.len(), 1);
        assert_eq!(result.coefficients[0].0, 1);
    }

    #[test]
    #[should_panic]
    fn test_div_not_possible() {
        let field = Field::new(7);

        let coefficients1 = vec![
            FieldElement::new(1, field),
            FieldElement::new(2, field),
            FieldElement::new(3, field),
        ];

        let coefficients2 = vec![
            FieldElement::new(2, field),
            FieldElement::new(3, field),
            FieldElement::new(4, field),
        ];

        let polynomial1 = Polynomial::new_from_coefficients(coefficients1);
        let polynomial2 = Polynomial::new_from_coefficients(coefficients2);

        let _ = polynomial1 / polynomial2;
    }

    #[test]
    fn test_div_assign() {
        let field = Field::new(7);

        let coefficients1 = vec![
            FieldElement::new(1, field),
            FieldElement::new(2, field),
            FieldElement::new(3, field),
        ];

        let coefficients2 = vec![
            FieldElement::new(1, field),
            FieldElement::new(2, field),
            FieldElement::new(3, field),
        ];

        let mut polynomial1 = Polynomial::new_from_coefficients(coefficients1);
        let polynomial2 = Polynomial::new_from_coefficients(coefficients2);

        polynomial1 /= polynomial2;

        assert_eq!(polynomial1.coefficients.len(), 1);
        assert_eq!(polynomial1.coefficients[0].0, 1);
    }

    #[test]
    #[should_panic]
    fn test_div_assign_not_possible() {
        let field = Field::new(7);

        let coefficients1 = vec![
            FieldElement::new(1, field),
            FieldElement::new(2, field),
            FieldElement::new(3, field),
        ];

        let coefficients2 = vec![
            FieldElement::new(2, field),
            FieldElement::new(3, field),
            FieldElement::new(4, field),
        ];

        let mut polynomial1 = Polynomial::new_from_coefficients(coefficients1);
        let polynomial2 = Polynomial::new_from_coefficients(coefficients2);

        polynomial1 /= polynomial2;
    }

    #[test]
    fn test_mono_compose() {
        let field = Field::new(7);

        let coefficients = vec![
            FieldElement::new(1, field),
            FieldElement::new(2, field),
            FieldElement::new(3, field),
        ];

        let polynomial = Polynomial::new_from_coefficients(coefficients);

        let mono = FieldElement::new(4, field);

        let polynomial = polynomial.compose(mono);

        assert_eq!(polynomial.coefficients[0].0, 1);
        assert_eq!(polynomial.coefficients[1].0, 1);
        assert_eq!(polynomial.coefficients[2].0, 6);
    }
}
