// Testing for correct results
// Benchmarking is done using criterion


// What to look for in those tests: floating-point errors.
// We might need to use an approximative equality test such as
// a == b <=> (a-b) < smallnumber

use fastiron::mc::mc_vector::MCVector;

#[test]
fn add() {
    let uu = MCVector {x: 1.0, y: 1.0, z: 1.0};
    let vv = MCVector {x: 1.0, y: 1.0, z: 1.0};
    let ww = MCVector {x: 2.0, y: 2.0, z: 2.0};
    assert_eq!(uu + vv, ww);
}

#[test]
fn add_assign() {
    let mut uu = MCVector {x: 1.0, y: 1.0, z: 1.0};
    let vv = MCVector {x: 1.0, y: 1.0, z: 1.0};
    let ww = MCVector {x: 2.0, y: 2.0, z: 2.0};
    uu += vv;
    assert_eq!(uu, ww);
}

#[test]
fn sub() {
    let uu = MCVector {x: 1.0, y: 1.0, z: 1.0};
    let vv = MCVector {x: 1.0, y: 1.0, z: 1.0};
    let ww = MCVector {x: 0.0, y: 0.0, z: 0.0};
    assert_eq!(uu - vv, ww);
}

#[test]
fn sub_assign() {
    let mut uu = MCVector {x: 1.0, y: 1.0, z: 1.0};
    let vv = MCVector {x: 1.0, y: 1.0, z: 1.0};
    let ww = MCVector {x: 0.0, y: 0.0, z: 0.0};
    uu -= vv;
    assert_eq!(uu, ww);
}

#[test]
fn mul() {
    let uu = MCVector {x: 1.0, y: 1.0, z: 1.0};
    let f = 2.0;
    let ww = MCVector {x: 2.0, y: 2.0, z: 2.0};
    assert_eq!(uu*f, ww);
}

#[test]
fn mul_assign() {
    let mut uu = MCVector {x: 1.0, y: 1.0, z: 1.0};
    let f = 2.0;
    let ww = MCVector {x: 2.0, y: 2.0, z: 2.0};
    uu *= f;
    assert_eq!(uu, ww);
}

#[test]
fn div_assign() {
    let mut uu = MCVector {x: 1.0, y: 1.0, z: 1.0};
    let f = 2.0;
    let ww = MCVector {x: 0.5, y: 0.5, z: 0.5};
    uu /= f;
    assert_eq!(uu, ww);
}

#[test]
#[should_panic]
fn div_assign_zero() {
    let mut uu = MCVector {x: 1.0, y: 1.0, z: 1.0};
    uu /= 0.0;
}