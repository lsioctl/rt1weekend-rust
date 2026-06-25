use std::ops;

// use crate::vector3;

// Floating point types such as f32 and f64 implement only PartialEq but not Eq because NaN != NaN.
// Clone and copy because I think it's cheap
// And I don't want:
//  * to implement operators for litterals or references
//  * use a macro to do it like in here:
//  https://github.com/ryankaplan/vec3/blob/master/src/lib.rs
// TODO: it may be cheap but it could be repeated a lot by pixel,
// so worth doing something more optimized
//  see also:
// https://stackoverflow.com/questions/28005134/how-do-i-implement-the-add-trait-for-a-reference-to-a-struct
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl ops::Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Self::Output {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Sub<Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Vector3) -> Self::Output {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector3 {
            x: rhs * self.x,
            y: rhs * self.y,
            z: rhs * self.z,
        }
    }
}

impl ops::Mul<Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Self::Output {
        rhs * self
    }
}

impl ops::Div<f64> for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: f64) -> Self::Output {
        (1. / rhs) * self
    }
}

pub fn dot(lhs: Vector3, rhs: Vector3) -> f64 {
    lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z
}

// fn cross(lhs: Vector3, rhs: Vector3) -> Vector3 {
//     Vector3 {
//         x: lhs.y * rhs.z - rhs.y * lhs.z,
//         y: rhs.x * lhs.z - lhs.x * rhs.z,
//         z: lhs.x * rhs.y - rhs.x * lhs.y,
//     }
// }

pub fn len_squared(v: Vector3) -> f64 {
    v.x.powi(2) + v.y.powi(2) + v.z.powi(2)
}

fn len(v: Vector3) -> f64 {
    len_squared(v).sqrt()
}

pub fn unit(v: Vector3) -> Vector3 {
    v / len(v)
}

#[cfg(test)]
mod tests {
    // importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        let v1 = Vector3 {
            x: 1.,
            y: 2.,
            z: 3.,
        };
        let v2 = Vector3 {
            x: 5.,
            y: 6.,
            z: 7.,
        };
        let v3 = Vector3 {
            x: 8.,
            y: 9.,
            z: 10.,
        };

        let v4 = Vector3 {
            x: v1.x + v2.x,
            y: v1.y + v2.y,
            z: v1.z + v2.z,
        };

        let v5 = Vector3 {
            x: v3.x + v4.x,
            y: v3.y + v4.y,
            z: v3.z + v4.z,
        };

        assert_eq!(v1 + v2, v4);
        assert_eq!(v1 + v2 + v3, v5);
    }

    #[test]
    fn test_sub() {
        let v1 = Vector3 {
            x: 1.,
            y: 2.,
            z: 3.,
        };
        let v2 = Vector3 {
            x: 5.,
            y: 6.,
            z: 7.,
        };
        let v3 = Vector3 {
            x: 8.,
            y: 9.,
            z: 10.,
        };

        let v4 = Vector3 {
            x: v1.x - v2.x,
            y: v1.y - v2.y,
            z: v1.z - v2.z,
        };

        let v5 = Vector3 {
            x: v4.x - v3.x,
            y: v4.y - v3.y,
            z: v4.z - v3.z,
        };

        assert_eq!(v1 - v2, v4);
        assert_eq!(v1 - v2 - v3, v5);
    }

    #[test]
    fn test_mul() {
        let v1 = Vector3 {
            x: 1.,
            y: 2.,
            z: 3.,
        };
        let coeff = 5.;
        let target = Vector3 {
            x: coeff * v1.x,
            y: coeff * v1.y,
            z: coeff * v1.z,
        };

        // here copies of v1 and coeff are done
        assert_eq!(coeff * v1, target);
        assert_eq!(v1 * coeff, target);
    }

    #[test]
    fn test_dot() {
        let v1 = Vector3 {
            x: 2.,
            y: 3.,
            z: 5.,
        };
        let v2 = Vector3 {
            x: 6.,
            y: 7.,
            z: 8.,
        };

        assert_eq!(dot(v1, v2), v1.x * v2.x + v1.y * v2.y + v1.z * v2.z);
    }

    #[test]
    fn test_cross() {
        let v1 = Vector3 {
            x: 2.,
            y: 3.,
            z: 5.,
        };
        let v2 = Vector3 {
            x: 6.,
            y: 7.,
            z: 8.,
        };

        let v3 = cross(v1, v2);

        let target = Vector3 {
            x: v1.y * v2.z - v2.y * v1.z,
            y: v2.x * v1.z - v1.x * v2.z,
            z: v1.x * v2.y - v2.x * v1.y,
        };

        assert_eq!(v3, target);
    }

    #[test]
    fn test_cross2() {
        let index = Vector3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        };
        let major = Vector3 {
            x: -1.0,
            y: 0.0,
            z: 0.0,
        };
        let thumb = Vector3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };

        let v3 = cross(index, major);

        assert_eq!(v3, thumb);
    }

    #[test]
    fn test_unit() {
        let v1 = Vector3 {
            x: 20.,
            y: 20.,
            z: 20.,
        };

        println!("{:?}", unit(v1));
        println!("{}", len(v1));
    }
}
