use std::ops;

// Floating point types such as f32 and f64 implement only PartialEq but not Eq because NaN != NaN.
// Clone and copy because I think it's cheap
// And I don't want:
//  * to implement operators for litterals or references
//  * use a macro to do it like in here:
//  https://github.com/ryankaplan/vec3/blob/master/src/lib.rs
#[derive(Debug, PartialEq, Clone, Copy)]
struct Vector3 {
    x: f64,
    y: f64,
    z: f64
}

impl ops::Add<&Vector3> for &Vector3 {
    type Output = Vector3;

    fn add(self, rhs: &Vector3) -> Vector3 {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl ops::Mul<f64> for &Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f64) -> Vector3 {
        Vector3 {
            x: rhs * self.x,
            y: rhs * self.y,
            z: rhs * self.z
        }
    }
}

impl ops::Mul<&Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, rhs: &Vector3) -> Vector3 {
        rhs * self
    }
}


#[cfg(test)]
mod tests {
    // importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        let v1 = Vector3 {x: 1., y: 2., z: 3.};
        let v2 = Vector3 {x: 5., y: 6., z: 7.};
        // let v3 = Vector3 {x: 8., y: 9., z: 10.};

        let v4 = Vector3 {
            x: v1.x + v2.x,
            y: v1.y + v2.y,
            z: v1.z + v2.z
        };

        // let v5 = Vector3 {
        //     x: v3.x + v4.x,
        //     y: v3.y + v4.y,
        //     z: v3.z + v4.z,
        // };

        assert_eq!(&v1 + &v2, v4);
        // can´t chain for now, I return a vector not a vector ref
        // assert_eq!(&v1 + &v2 + &v3, v5);
    }

    #[test]
    fn test_mul() {
        let v1 = Vector3 {x: 1., y: 2., z: 3.};
        let coeff = 5.;
        let target = Vector3 {
            x: coeff * v1.x,
            y: coeff * v1.y,
            z: coeff * v1.z,
        };

        // here copies of v1 and coeff are done
        assert_eq!(coeff * &v1, target);
        assert_eq!(&v1 * coeff, target);


    }
}