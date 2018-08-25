#![allow(dead_code)]

use std::cmp;
use std::ops;

use approx::AbsDiffEq;

// TODO: Make Tuple struct generic over numeric types
#[derive(Clone, Copy, Debug)]
pub struct Tuple {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

/// Might be better as a trait, with Vector and Point implementing it
impl Tuple {
    pub fn new(x: impl Into<f64>, y: impl Into<f64>, z: impl Into<f64>, w: impl Into<f64>) -> Self {
        Tuple {
            x: x.into(),
            y: y.into(),
            z: z.into(),
            w: w.into(),
        }
    }

    pub fn new_point(x: impl Into<f64>, y: impl Into<f64>, z: impl Into<f64>) -> Self {
        Tuple {
            x: x.into(),
            y: y.into(),
            z: z.into(),
            w: 1.0,
        }
    }

    pub fn new_vector(x: impl Into<f64>, y: impl Into<f64>, z: impl Into<f64>) -> Self {
        Tuple {
            x: x.into(),
            y: y.into(),
            z: z.into(),
            w: 0.0,
        }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn w(&self) -> f64 {
        self.w
    }

    pub fn get_normalized(&self) -> Self {
        let mag = self.magnitude();
        Tuple {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
            w: self.w / mag,
        }
    }

    pub fn dot(&self, other: &Self) -> f64 {
        (self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w)
    }

    pub fn cross(&self, other: &Self) -> Self {
        if !self.is_vector() || !other.is_vector() {
            panic!("Attempted to take cross product of non-vector Tuple");
        }
        Tuple {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.z * other.y - self.y * other.z,
            w: 0.0,
        }
    }

    pub fn magnitude(&self) -> f64 {
        (self.x().powi(2) + self.y().powi(2) + self.z().powi(2) + self.w().powi(2)).sqrt()
    }

    pub fn is_point(&self) -> bool {
        abs_diff_eq!(self.w(), 1.0)
    }

    pub fn is_vector(&self) -> bool {
        abs_diff_eq!(self.w(), 0.0)
    }
}

impl ops::Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Tuple {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl ops::Add for Tuple {
    type Output = Tuple;

    fn add(self, rhs: Self) -> Self::Output {
        Tuple {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl ops::Sub for Tuple {
    type Output = Tuple;

    fn sub(self, rhs: Self) -> Self::Output {
        Tuple {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

// TODO: gonna have to use macro_rules! here to allow integer
impl ops::Mul<f64> for Tuple {
    type Output = Tuple;

    fn mul(self, scalar: f64) -> Self::Output {
        Tuple {
            x: scalar * self.x,
            y: scalar * self.y,
            z: scalar * self.z,
            w: scalar * self.w,
        }
    }
}

impl ops::Div<f64> for Tuple {
    type Output = Tuple;

    fn div(self, scalar: f64) -> Self::Output {
        Tuple {
            x: scalar / self.x,
            y: scalar / self.y,
            z: scalar / self.z,
            w: scalar / self.w,
        }
    }
}

impl AbsDiffEq for Tuple {
    type Epsilon = f64;

    fn default_epsilon() -> Self::Epsilon {
        1.0e-6
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        abs_diff_eq!(self.x, other.x, epsilon = epsilon)
            || abs_diff_eq!(self.y, other.y, epsilon = epsilon)
            || abs_diff_eq!(self.z, other.z, epsilon = epsilon)
            || abs_diff_eq!(self.w, other.w, epsilon = epsilon)
    }
}

impl cmp::PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        abs_diff_eq!(self, other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tuple_with_w_1_is_a_point() {
        let tuple = Tuple::new_point(4.3, -4.2, 3.1);
        assert_eq!(tuple.x(), 4.3);
        assert_eq!(tuple.y(), -4.2);
        assert_eq!(tuple.z(), 3.1);
        assert!(tuple.is_point());
        assert!(!tuple.is_vector());
    }

    #[test]
    fn tuple_with_w_0_is_a_vector() {
        let tuple = Tuple::new_vector(4.3, -4.2, 3.1);
        assert_eq!(tuple.x(), 4.3);
        assert_eq!(tuple.y(), -4.2);
        assert_eq!(tuple.z(), 3.1);
        assert!(!tuple.is_point());
        assert!(tuple.is_vector());
    }

    #[test]
    fn adding_point_and_vector() {
        let point = Tuple::new_point(3, -2, 5);
        let vector = Tuple::new_vector(-2, 3, 1);
        let resulting_point = point + vector;
        assert_eq!(resulting_point, Tuple::new_point(1, 1, 6));
        assert!(resulting_point.is_point());
    }

    #[test]
    fn subtracting_two_points() {
        let point_1 = Tuple::new_point(3, 2, 1);
        let point_2 = Tuple::new_point(5, 6, 7);
        let resulting_vector = point_1 - point_2;
        assert_eq!(resulting_vector, Tuple::new_vector(-2, -4, -6));
        assert!(resulting_vector.is_vector());
    }

    #[test]
    fn subtracting_vector_from_point() {
        let point = Tuple::new_point(3, 2, 1);
        let vector = Tuple::new_vector(5, 6, 7);
        let resulting_point = point - vector;
        assert_eq!(resulting_point, Tuple::new_point(-2, -4, -6));
        assert!(resulting_point.is_point());
    }

    #[test]
    fn subtracting_two_vectors() {
        let vector_1 = Tuple::new_vector(3, 2, 1);
        let vector_2 = Tuple::new_vector(5, 6, 7);
        let resulting_vector = vector_1 - vector_2;
        assert_eq!(resulting_vector, Tuple::new_vector(-2, -4, -6));
        assert!(resulting_vector.is_vector());
    }

    #[test]
    fn subtracting_vector_from_zero_vector() {
        let zero = Tuple::new_vector(0, 0, 0);
        let vector = Tuple::new_vector(1, -2, 3);
        let resulting_vector = zero - vector;
        assert_eq!(resulting_vector, Tuple::new_vector(-1, 2, -3));
    }

    #[test]
    fn negating_tuple() {
        let tuple = Tuple::new(1, -2, 3, -4);
        assert_eq!(-tuple, Tuple::new(-1, 2, -3, 4));
    }

    #[test]
    fn multiplying_tuple_by_scalar() {
        let tuple = Tuple::new(1, -2, 3, -4);
        assert_eq!(tuple * 3.5, Tuple::new(3.5, -7, 10.5, -14));
    }

    #[test]
    fn multiplying_tuple_by_fraction() {
        let tuple = Tuple::new(1, -2, 3, -4);
        assert_eq!(tuple * 0.5, Tuple::new(0.5, -1, 1.5, -2));
    }

    #[test]
    fn dividing_tuple_by_scalar() {
        let tuple = Tuple::new(1, -2, 3, -4);
        assert_eq!(tuple / 2.0, Tuple::new(0.5, -1, 1.5, -2));
    }

    #[test]
    fn magnitude_of_unit_vectors() {
        let v = Tuple::new_vector(1, 0, 0);
        assert_abs_diff_eq!(v.magnitude(), 1.0);
        let v = Tuple::new_vector(0, 1, 0);
        assert_abs_diff_eq!(v.magnitude(), 1.0);
        let v = Tuple::new_vector(0, 0, 1);
        assert_abs_diff_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn magnitude_of_vectors() {
        let v = Tuple::new_vector(1, 2, 3);
        assert_abs_diff_eq!(v.magnitude(), 14.0_f64.sqrt());
        let v = Tuple::new_vector(-1, -2, -3);
        assert_abs_diff_eq!(v.magnitude(), 14.0_f64.sqrt());
    }

    #[test]
    fn normalizing_vectors() {
        let v = Tuple::new_vector(4, 0, 0);
        assert_eq!(v.get_normalized(), Tuple::new_vector(1, 0, 0));
        let v = Tuple::new_vector(1, 2, 3);
        assert_eq!(
            v.get_normalized(),
            Tuple::new_vector(
                1.0 / 14.0_f64.sqrt(),
                2.0 / 14.0_f64.sqrt(),
                3.0 / 14.0_f64.sqrt()
            )
        );
    }

    #[test]
    fn magnitude_of_normalized_vector() {
        let v = Tuple::new_vector(1, 2, 3);
        let norm = v.get_normalized();
        assert_abs_diff_eq!(norm.magnitude(), 1.0);
    }

    #[test]
    fn dot_product_of_tuples() {
        let v1 = Tuple::new_vector(1, 2, 3);
        let v2 = Tuple::new_vector(2, 3, 4);
        assert_eq!(v1.dot(&v2), 20.0);
    }

    #[test]
    fn cross_product_of_tuples() {
        let v1 = Tuple::new_vector(1, 2, 3);
        let v2 = Tuple::new_vector(2, 3, 4);
        assert_abs_diff_eq!(v1.cross(&v2), Tuple::new_vector(-1, 2, -1));
        assert_abs_diff_eq!(v2.cross(&v1), Tuple::new_vector(1, -2, 1));
    }
}
