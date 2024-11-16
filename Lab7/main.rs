use std::fmt;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Neg;
use std::ops::Sub;
use std::ops::SubAssign;
#[derive(Debug, Copy, Clone)]
struct Complex {
    real: f64,
    imag: f64,
}

impl Complex {
    fn new<T: Into<f64>, U: Into<f64>>(real: T, imag: U) -> Self {
        Complex {
            real: real.into(),
            imag: imag.into(),
        }
    }

    fn conjugate(&self) -> Self {
        Complex {
            real: self.real,
            imag: -self.imag,
        }
    }
}

impl From<i32> for Complex {
    fn from(value: i32) -> Self {
        Complex {
            real: value as f64,
            imag: 0.0,
        }
    }
}

impl From<f64> for Complex {
    fn from(value: f64) -> Self {
        Complex {
            real: value,
            imag: 0.0,
        }
    }
}

impl<T: Into<Complex>> Add<T> for Complex {
    type Output = Complex;

    fn add(self, other: T) -> Self::Output {
        let other = other.into();
        Complex {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }
}

impl<T: Into<Complex>> Sub<T> for Complex {
    type Output = Complex;

    fn sub(self, other: T) -> Self::Output {
        let other = other.into();
        Complex {
            real: self.real - other.real,
            imag: self.imag - other.imag,
        }
    }
}

impl<T: Into<Complex>> Mul<T> for Complex {
    type Output = Complex;

    fn mul(self, other: T) -> Self::Output {
        let other = other.into();
        Complex {
            real: self.real * other.real - self.imag * other.imag,
            imag: self.real * other.imag + self.imag * other.real,
        }
    }
}

impl Neg for Complex {
    type Output = Complex;

    fn neg(self) -> Self::Output {
        Complex {
            real: -self.real,
            imag: -self.imag,
        }
    }
}
impl PartialEq for Complex {
    fn eq(&self, other: &Self) -> bool {
        eq_rel(self.real, other.real) && eq_rel(self.imag, other.imag)
    }
}
impl<T: Into<Complex>> AddAssign<T> for Complex {
    fn add_assign(&mut self, other: T) {
        let other = other.into();
        self.real += other.real;
        self.imag += other.imag;
    }
}
impl<T: Into<Complex>> SubAssign<T> for Complex {
    fn sub_assign(&mut self, other: T) {
        let other = other.into();
        self.real -= other.real;
        self.imag -= other.imag;
    }
}
impl<T: Into<Complex>> MulAssign<T> for Complex {
    fn mul_assign(&mut self, other: T) {
        let other = other.into();
        self.real = self.real * other.real - self.imag * other.imag;
        self.imag = self.real * other.imag + self.imag * other.real;
    }
}
impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.real, self.imag) {
            (0.0, 0.0) => write!(f, "0"),
            (r, 0.0) => write!(f, "{}", r),
            (0.0, i) => write!(f, "{}i", i),
            (r, i) if i > 0.0 => write!(f, "{}+{}i", r, i),
            (r, i) => write!(f, "{}{}i", r, i),
        }
    }
}
fn eq_rel(x: f64, y: f64) -> bool {
    (x - y).abs() < 0.001
}

macro_rules! assert_eq_rel {
    ($x:expr, $y: expr) => {
        let x = $x as f64;
        let y = $y as f64;
        let r = eq_rel(x, y);
        assert!(r, "{} != {}", x, y);
    };
}

fn main() {
    let a = Complex::new(1.0, 2.0);
    assert_eq_rel!(a.real, 1.0);
    assert_eq_rel!(a.imag, 2.0);

    let b = Complex::new(2.0, 3.0);
    let c = a + b;
    assert_eq_rel!(c.real, 3.0);
    assert_eq_rel!(c.imag, 5.0);

    let d = c - a;
    assert_eq!(b, d);

    let e = (a * d).conjugate();
    assert_eq_rel!(e.imag, -7.0);

    let f = (a + b - d) * c;
    assert_eq!(f, Complex::new(-7.0, 11.0));

    assert_eq!(Complex::new(1.0, 2.0).to_string(), "1+2i");
    assert_eq!(Complex::new(1.0, -2.0).to_string(), "1-2i");
    assert_eq!(Complex::new(0.0, 5.0).to_string(), "5i");
    assert_eq!(Complex::new(7.0, 0.0).to_string(), "7");
    assert_eq!(Complex::new(0.0, 0.0).to_string(), "0");

    let h = Complex::new(-4.0, -5.0);
    let i = h - (h + Complex::new(5.0, 0.0)) * Complex::new(2.0, 0.0);
    assert_eq_rel!(i.real, -6.0);

    let j = -i + i;
    assert_eq_rel!(j.real, 0.0);
    assert_eq_rel!(j.imag, 0.0);

    let mut k = Complex::new(7.0, -2.0);
    k += 6.0;

    let mut l = Complex::new(7.0, -2.0);
    l += Complex::new(2.4, 5.0);

    let mut m = Complex::new(7.0, -2.0);
    m -= 6.0;

    let mut n = Complex::new(7.0, -2.0);
    n -= Complex::new(2.4, 5.0);

    let mut p = Complex::new(7.0, -2.0);
    p *= 6.0;

    let mut q = Complex::new(7.0, -2.0);
    q *= Complex::new(2.0, 5.0);

    println!("ok!");
}
