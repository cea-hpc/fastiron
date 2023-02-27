// Testing for correct results
// Benchmarking is done using criterion

// What to look for in those tests: floating-point errors.
// We might need to use an approximative equality test such as
// a == b <=> (a-b) < smallnumber

use fastiron::mc::mc_vector::MCVector;
use num::Float;

// Basic operations

#[test]
fn add() {
    let uu = MCVector {
        x: 1.0 / 3.0,
        y: 1.0343253253254332 / 3.0,
        z: -1.0 / 3.0,
    };
    let vv = MCVector {
        x: 2.0 / 3.0,
        y: 31.0 / 3.0,
        z: 2.0 / 3.0,
    };
    let ww = MCVector {
        x: 1.0,
        y: 32.03432532532543 / 3.0,
        z: 1.0 / 3.0,
    };
    assert_eq!(uu + vv, ww);
}

#[test]
fn add_assign() {
    let mut uu = MCVector {
        x: 1.0 / 3.0,
        y: 1.0343253253254332 / 3.0,
        z: -1.0 / 3.0,
    };
    let vv = MCVector {
        x: 2.0 / 3.0,
        y: 31.0 / 3.0,
        z: 2.0 / 3.0,
    };
    let ww = MCVector {
        x: 1.0,
        y: 32.03432532532543 / 3.0,
        z: 1.0 / 3.0,
    };
    uu += vv;
    assert_eq!(uu, ww);
}

#[test]
fn sub() {
    let uu = MCVector {
        x: 1.0 / 3.0,
        y: 1.0343253253254332,
        z: 1.0 / 3.0,
    };
    let vv = MCVector {
        x: 2.0 / 3.0,
        y: 31.0,
        z: -2.0 / 3.0,
    };
    // Exact equality work in this case, but this isn't
    // consistent. For example, dividing by 3 all y coords
    // will make it fail because of error propagation.
    let ww = MCVector {
        x: -1.0 / 3.0,
        //y: -29.9656746746745668,
        //y: -29.965674674674566,
        y: -29.965674674674567,
        z: 1.0,
    };
    assert_eq!(uu - vv, ww);
}

#[test]
fn sub_assign() {
    let mut uu = MCVector {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };
    let vv = MCVector {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };
    let ww = MCVector {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    uu -= vv;
    assert_eq!(uu, ww);
}

#[test]
fn mul() {
    let uu = MCVector {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };
    let f = 2.0;
    let ww = MCVector {
        x: 2.0,
        y: 2.0,
        z: 2.0,
    };
    assert_eq!(uu * f, ww);
}

#[test]
fn mul_assign() {
    let mut uu = MCVector {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };
    let f = 2.0;
    let ww = MCVector {
        x: 2.0,
        y: 2.0,
        z: 2.0,
    };
    uu *= f;
    assert_eq!(uu, ww);
}

#[test]
fn div_assign() {
    let mut uu = MCVector {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };
    let f = 2.0;
    let ww = MCVector {
        x: 0.5,
        y: 0.5,
        z: 0.5,
    };
    uu /= f;
    assert_eq!(uu, ww);
}

#[test]
#[should_panic]
fn div_assign_zero() {
    let mut uu = MCVector {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };
    uu /= 0.0;
}

// Pushing the boundaries

#[test]
fn floating_point_error() {
    let uu = MCVector {
        x: 1.0 / 3.0,
        y: 1.0343253253254332 / 3.0,
        z: 1.0 / 3.0,
    };
    let vv = MCVector {
        x: 2.0 / 3.0,
        y: 31.0 / 3.0,
        z: -2.0 / 3.0,
    };
    let ww = MCVector {
        x: -1.0 / 3.0,
        //y: -29.9656746746745668/3.0, // error with exact value
        //y: -29.965674674674566/3.0, // error with truncated value
        y: -29.965674674674567 / 3.0, // error with rounded value
        z: 1.0,
    };
    // instead of checking for exact equality, we check that the
    // difference is close enough to zero
    assert!((uu - vv).is_almost_equal(&ww));
}

// Methods

#[test]
pub fn length() {
    // trivial case
    let mut uu = MCVector {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };
    assert_eq!(uu.length(), 3.0.sqrt());

    // negative & non rational coordinates
    uu.x = -1.0;
    uu.y = 23.0.sqrt();
    assert_eq!(uu.length(), 5.0);
}

#[test]
pub fn distance() {
    let uu = MCVector {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };
    let ww = MCVector {
        x: 2.0,
        y: 2.0,
        z: 2.0,
    };
    assert_eq!(uu.distance(&ww), 3.0.sqrt());
}

#[test]
pub fn dot_product() {
    let uu = MCVector {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };
    let ww = MCVector {
        x: 2.0,
        y: 2.0,
        z: 2.0,
    };
    assert_eq!(uu.dot(&ww), 6.0);
}

#[test]
pub fn cross_product() {
    let uu = MCVector {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };
    let ww = MCVector {
        x: 2.0,
        y: 2.0,
        z: 2.0,
    };
    assert_eq!(uu.cross(&ww), MCVector::default()); // default is the zero element
}
