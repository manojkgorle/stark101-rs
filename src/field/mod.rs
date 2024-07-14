use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
// Define a field
// Define a field element
// Define arithmetic operations on field elements
#[derive(Debug, Clone, Copy)]
pub struct Field(pub u64);

impl Field {
    pub fn new(x: u64) -> Field {
        Field(x)
    }
}

impl PartialEq for Field {
    fn eq(&self, other: &Field) -> bool {
        self.0 == other.0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FieldElement(pub u64, pub Field);

impl FieldElement {
    pub fn new(x: u64, field: Field) -> FieldElement {
        FieldElement(x % field.0, field)
    }

    pub fn zero(field: Field) -> FieldElement {
        FieldElement(0, field)
    }

    pub fn one(field: Field) -> FieldElement {
        FieldElement(1, field)
    }

    pub fn modulus(&self) -> u64 {
        self.1 .0
    }

    pub fn inverse(&self) -> FieldElement {
        let mut inv = 1;
        let mut base = self.0;
        let mut exp = self.1 .0 - 2;
        while exp > 0 {
            if exp % 2 == 1 {
                inv = (inv * base) % self.1 .0;
            }
            base = (base * base) % self.1 .0;
            exp /= 2;
        }
        FieldElement(inv, self.1)
    }

    pub fn pow(&self, exp: u64) -> FieldElement {
        let mut res = 1;
        let mut base = self.0;
        let mut exp = exp;
        while exp > 0 {
            if exp % 2 == 1 {
                res = (res * base) % self.1 .0;
            }
            base = (base * base) % self.1 .0;
            exp /= 2;
        }
        FieldElement(res % self.1 .0, self.1)
    }
}

impl Add for FieldElement {
    type Output = FieldElement;
    fn add(self, other: FieldElement) -> FieldElement {
        if self.1 != other.1 {
            panic!("Fields must be same");
        }
        FieldElement((self.0 + other.0) % self.1 .0, self.1)
    }
}

impl AddAssign for FieldElement {
    fn add_assign(&mut self, other: FieldElement) {
        if self.1 != other.1 {
            panic!("Fields must be same");
        }
        self.0 = (self.0 + other.0) % self.1 .0;
    }
}

impl Sub for FieldElement {
    type Output = FieldElement;
    fn sub(self, other: FieldElement) -> FieldElement {
        if self.1 != other.1 {
            panic!("Fields must be same");
        }
        if self.0 < other.0 {
            FieldElement((self.0 + self.1 .0 - other.0) % self.1 .0, self.1)
        } else {
            FieldElement((self.0 - other.0) % self.1 .0, self.1)
        }
    }
}

impl SubAssign for FieldElement {
    fn sub_assign(&mut self, other: FieldElement) {
        if self.1 != other.1 {
            panic!("Fields must be same");
        }
        if self.0 < other.0 {
            self.0 = (self.0 + self.1 .0 - other.0) % self.1 .0;
        } else {
            self.0 = (self.0 - other.0) % self.1 .0;
        }
    }
}

impl Mul for FieldElement {
    type Output = FieldElement;
    fn mul(self, other: FieldElement) -> FieldElement {
        if self.1 != other.1 {
            panic!("Fields must be same");
        }
        FieldElement((self.0 * other.0) % self.1 .0, self.1)
    }
}

impl MulAssign for FieldElement {
    fn mul_assign(&mut self, other: FieldElement) {
        if self.1 != other.1 {
            panic!("Fields must be same");
        }
        self.0 = (self.0 * other.0) % self.1 .0;
    }
}

impl Div for FieldElement {
    type Output = FieldElement;
    fn div(self, other: FieldElement) -> FieldElement {
        if self.1 != other.1 {
            panic!("Fields must be same");
        }
        let mut inv = 1;
        let mut base = other.0;
        let mut exp = self.1 .0 - 2;
        while exp > 0 {
            if exp % 2 == 1 {
                inv = (inv * base) % self.1 .0;
            }
            base = (base * base) % self.1 .0;
            exp /= 2;
        }
        FieldElement((self.0 * inv) % self.1 .0, self.1)
    }
}

impl DivAssign for FieldElement {
    fn div_assign(&mut self, other: FieldElement) {
        if self.1 != other.1 {
            panic!("Fields must be same");
        }
        let mut inv = 1;
        let mut base = other.0;
        let mut exp = self.1 .0 - 2;
        while exp > 0 {
            if exp % 2 == 1 {
                inv = (inv * base) % self.1 .0;
            }
            base = (base * base) % self.1 .0;
            exp /= 2;
        }
        self.0 = (self.0 * inv) % self.1 .0;
    }
}

impl Neg for FieldElement {
    type Output = FieldElement;
    fn neg(self) -> FieldElement {
        FieldElement(self.1 .0 - self.0, self.1)
    }
}
#[cfg(test)]
mod test_field_operations {
    use super::*;

    #[test]
    fn test_field_add() {
        let field = Field::new(7);
        let a = FieldElement::new(1, field);
        let b = FieldElement::new(2, field);
        let c = a + b;
        assert_eq!(c.0, 3);
    }

    #[test]
    fn test_field_sub() {
        let field = Field::new(7);
        let a = FieldElement::new(1, field);
        let b = FieldElement::new(2, field);
        let c = a - b;
        assert_eq!(c.0, 6);
    }

    #[test]
    fn test_field_mul() {
        let field = Field::new(7);
        let a = FieldElement::new(1, field);
        let b = FieldElement::new(2, field);
        let c = a * b;
        assert_eq!(c.0, 2);
    }

    #[test]
    fn test_field_div() {
        let field = Field::new(7);
        let a = FieldElement::new(1, field);
        let b = FieldElement::new(2, field);
        let c = a / b;
        assert_eq!(c.0, 4);
    }

    #[test]
    fn test_field_inverse() {
        let field = Field::new(7);
        let a = FieldElement::new(2, field);
        let b = a.inverse();
        assert_eq!(b.0, 4);
    }

    #[test]
    fn test_field_pow() {
        let field = Field::new(7);
        let a = FieldElement::new(2, field);
        let b = a.pow(3);
        assert_eq!(b.0, 1);
    }

    #[test]
    #[should_panic]
    fn test_diff_field() {
        let field1 = Field::new(7);
        let field2 = Field::new(8);
        let a = FieldElement::new(1, field1);
        let b = FieldElement::new(2, field2);
        let _ = a + b;
    }
}
