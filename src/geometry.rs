use std::ops::{Index, IndexMut};
use std::convert::From;
use std::ops::{Add, Div, Mul, Neg, Sub};

use num_traits::{Float, Num};

#[derive(Debug,Copy,Clone)]
pub struct GVec<T, const D: usize> ([T; D]);

#[derive(Debug)]
pub struct GVec3<T> {
    x: T,
    y: T,
    z: T
}

impl<T, const D: usize> From<[T; D]> for GVec<T, D>{
    fn from(v: [T; D]) -> Self {
        GVec(v)
    }
}

impl<T, const D: usize> Index<usize> for GVec<T, D> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < D);
        &self.0[index]
    }
}

impl<T, const D: usize> IndexMut<usize> for GVec<T, D> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < D);
        &mut self.0[index]
    }
}

impl<T: Float, const D: usize> GVec<T, D> {
    pub fn norm(self) -> T {
        match D {
            1.. => (self.0.into_iter().reduce(|acc, i| acc + i*i)).unwrap().sqrt(),
            _ => T::zero()
        } 
    }

    pub fn normalize(self) -> Self {
        self * (T::one() / self.norm())
    }
}

impl<T: Num+Copy, const D: usize> Mul<T> for GVec<T, D> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self(self.0.map(|x| x*rhs))
    }
}

impl<T: Num, const D: usize> Mul for GVec<T, D> {
    type Output = T;

    fn mul(self, rhs: Self) -> Self::Output {
        self.0.into_iter()
        .zip(rhs.0.into_iter())
        .fold(T::zero(), |a, x| a + x.0*x.1)
    }
}

impl<T:Num+Copy, const D: usize> Add for GVec<T, D> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut arr = self.0;
        for i in 0..D {
            arr[i] = arr[i] + rhs.0[i];
        }
        Self(
            arr
        )
    }
}

impl<T:Num+Copy, const D: usize> Sub for GVec<T, D> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut arr = self.0;
        for i in 0..D {
            arr[i] = arr[i] - rhs.0[i];
        }
        Self(
            arr
        )
    }
}

impl<T: Num+Copy+Neg<Output = T>, const D: usize> Neg for GVec<T,D> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self*(-T::one())
    }
}

impl<T> Index<usize> for GVec3<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < 3);
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => unreachable!()
        }
    }
}

impl<T> IndexMut<usize> for GVec3<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < 3);
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => unreachable!()
        }
    }
}

impl<T: Float> GVec3<T> {
    pub fn norm(&self) -> T {
        (self.x*self.x + self.y*self.y + self.z*self.z).sqrt()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn disp() {
        let v = GVec::from([0.0, 1.0, -1.0]);
        println!("{:?}", v);
    }

    #[test]
    fn index() {
        let v = GVec::from([0.0, 1.0, -1.0]);
        assert_eq!(v[1], 1.0);
    }

    #[test]
    #[should_panic]
    fn out_of_bound() {
        let v = GVec::from([0.0, 1.0, -1.0]);
        v[4];
    }

    #[test]
    #[should_panic]
    fn out_of_bound_mut() {
        let mut v = GVec::from([0.0, 1.0, -1.0]);
        v[4] = 5.0;
    }

    #[test]
    fn norm3() {
        let v = GVec::from([0.0, 1.0, -1.0]);
        println!("{}", v.norm());
        assert_eq!((v.norm() - 2.0.sqrt()).abs() < 1e-5, true)
    }

    #[test]
    fn normalize3() {
        let v = GVec::from([0.0, 1.0, -1.0]);
        let n = v.normalize();
        println!("v {:?} {}\nn {:?} {}", v, v.norm(), n, n.norm());
        assert!((n.norm()) - 1.0 < 1e-5)
    }

    #[test]
    fn vec_scalar_mul() {
        let v = GVec::from([0.0, 1.0, -1.0]);
        let v2 = v*2.0;
        println!("v {:?} v*2.0 {:?}", v, v2);
        for i in 0..3 {
            assert!(v2[i] - v[i]*2.0 < 1e-5);
        }
        //let v2 = v*2;
        println!("v {:?} v*2.0 {:?}", v, v2);
        for i in 0..v.0.len() {
            assert!(v2[i] - v[i]*2.0 < 1e-5);
        }
    }

    #[test]
    fn vec_vec_mul() {
        let v = GVec::from([0.0, 1.0, -1.0]);
        let v2 = v*2.0;
        let vmul = v*v2;
        println!("v {:?} v*2.0 {:?} v*v2 {:?}", v, v2, vmul);
        assert_eq!(vmul, 4.0);
    }

    #[test]
    fn vec_vec_add() {
        let v = GVec::from([0.0, 1.0, -1.0]);
        let v2 = v*2.0;
        let vadd = v+v2;
        println!("v {:?} v*2.0 {:?} v+v2 {:?}", v, v2, vadd);
        for i in 0..vadd.0.len() {
            assert!(v2[i] + v[i] - vadd[i] < 1e-5);
        }
    }

    #[test]
    fn vec_vec_sub() {
        let v = GVec::from([0.0, 1.0, -1.0]);
        let v2 = v*2.0;
        let vadd = v-v2;
        println!("v {:?} v*2.0 {:?} v-v2 {:?}", v, v2, vadd);
        for i in 0..vadd.0.len() {
            assert!(v2[i] - v[i] + vadd[i] < 1e-5);
        }
    }

    #[test]
    fn vec_neg() {
        let v = GVec::from([0.0, 1.0, -1.0]);
        let vn = -v;
        println!("v {:?} -v {:?}", v, vn);
        for i in 0..v.0.len() {
            assert!(vn[i] + v[i] < 1e-5);
        }
    }
}