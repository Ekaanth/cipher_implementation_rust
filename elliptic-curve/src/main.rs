use std::fmt::{Display, Result};
#[derive(Debug)]
struct Fp {
    value: u32,
    modulus: u32,
}

trait Field {
    fn new(v: u32, m: u32) -> Option<Self>
    where
        Self: Sized;
    fn check_input(&self, other: &Self) -> bool;
    fn add(&self, other: &Self) -> Option<Self>
    where
        Self: Sized;
    fn sub(&self, other: &Self) -> Option<Self>
    where
        Self: Sized;
    fn mul(&self, other: &Self) -> Option<Self>
    where
        Self: Sized;
    fn inv(&self) -> Option<Self>
    where
        Self: Sized;
    fn sq(&self) -> Option<Self>
    where
        Self: Sized;
    fn extended_euclidean(a: i32, b: i32) -> (i32, i32, i32);
    // fn pow(self: &Self, other: &Self) -> Option<Self>
    // where
    //     Self: Sized;
}

impl Display for Fp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        write!(f, "{}mod{}", self.value, self.modulus)
    }
}

impl Field for Fp {
    fn new(v: u32, m: u32) -> Option<Self> {
        // check if m is prime
        if m <= 1 {
            return None;
        }

        let mut i: u32 = 2;
        while i * i <= m {
            if m % i == 0 {
                return None;
            }
            i += 1;
        }
        Some(Self {
            value: v,
            modulus: m,
        })
    }

    fn check_input(self: &Self, other: &Self) -> bool {
        if self.value < self.modulus && other.value < other.modulus && self.modulus == other.modulus
        {
            true
        } else {
            false
        }
    }

    fn add(self: &Self, other: &Self) -> Option<Self> {
        if Self::check_input(&self, &other) {
            let value = (self.value + other.value) % self.modulus;
            Some(Fp {
                value: value,
                modulus: self.modulus,
            })
        } else {
            None
        }
    }

    fn sub(self: &Self, other: &Self) -> Option<Self> {
        if Self::check_input(&self, &other) {
            let value;
            if self.value < other.value {
                value = other.value - self.value;
            } else {
                value = self.value - other.value;
            }
            Some(Fp {
                value: value,
                modulus: self.modulus,
            })
        } else {
            None
        }
    }

    fn mul(self: &Self, other: &Self) -> Option<Self> {
        if Self::check_input(&self, &other) {
            let value = (self.value * other.value) % self.modulus;
            Some(Fp {
                value: value,
                modulus: self.modulus,
            })
        } else {
            None
        }
    }

    fn inv(self: &Self) -> Option<Self> {
        let (_gcd, mut u, _v) = Self::extended_euclidean(self.value as i32, self.modulus as i32);
        let modulus: i32 = (self.modulus as u32).try_into().unwrap();
        if u < 0 {
            u = u + modulus;
        }
        Some(Fp {
            value: u as u32,
            modulus: self.modulus,
        })
    }

    fn extended_euclidean(a: i32, b: i32) -> (i32, i32, i32) {
        if b == 0 {
            return (a, 1, 0);
        }
        let (gcd, x, y) = Self::extended_euclidean(b, a % b);
        return (gcd, y, x - (a / b) * y);
    }

    fn sq(&self) -> Option<Self> {
        Some(Self::mul(&self, &self).unwrap())
    }

    // fn pow(self: &Self, other: &Self) -> Option<Self> {
    //     if Self::check_input(&self, &other) {
    //         let value = (u32::pow(self.value, other.value as u32)) % self.modulus;
    //         Some(Fp {
    //             value: value,
    //             modulus: self.modulus,
    //         })
    //     } else {
    //         None
    //     }
    // }

    // fn sqrt(self: &Self) -> Option<Self> {

    // }
}
fn main() {
    let f1 = Fp::new(10, 41).unwrap();
    let f2 = Fp::new(11, 41).unwrap();


    match f1.add(&f2) {
        None => {
            println!("error: invalid input");
        }
        Some(f) => {
            println!("{} + {} = {}", f1, f2, f);
        }
    }

    match f1.sub(&f2) {
        None => {
            println!("error: invalid input");
        }
        Some(f) => {
            println!("{} - {} = {}", f1, f2, f);
        }
    }

    match f1.mul(&f2) {
        None => {
            println!("error: invalid input");
        }
        Some(f) => {
            println!("{} x {} = {}", f1, f2, f);
        }
    }

    println!("inverse of {} = {}", f1, f1.inv().unwrap());
    println!("square of {} = {}", f1, f1.sq().unwrap());

}
