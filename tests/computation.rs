// Tests used to compare the results of certain computation heavy
// functions with their result in the original code.
// Results are hard coded.

use fastiron::{
    collision_event::update_trajectory,
    constants::physical::SMALL_FLOAT,
    direction_cosine::DirectionCosine,
    mc::{
        mc_particle::MCParticle,
        mc_rng_state::{pseudo_des, rng_sample, spawn_rn_seed},
        mc_vector::MCVector,
    },
};
use num::Float;

#[test]
pub fn rng_spawned_number() {
    let mut seed: u64 = 90374384094798327;
    let res = spawn_rn_seed::<f64>(&mut seed);

    assert_eq!(res, 3246986314100353546);
}

#[test]
pub fn pseudo_hash() {
    let mut a: u32 = 123214124;
    let mut b: u32 = 968374242;
    pseudo_des(&mut a, &mut b);

    assert_eq!(a, 702007026);
    assert_eq!(b, 3221367323);
}

#[test]
pub fn sample_isotropic() {
    let mut dd: DirectionCosine<f64> = DirectionCosine {
        alpha: 0.2140,
        beta: 0.8621,
        gamma: 0.7821,
    };
    let mut seed: u64 = 90374384094798327;
    dd.sample_isotropic(&mut seed);

    assert_eq!(dd.alpha, 0.9083218129645693);
    assert_eq!(dd.beta, -0.3658911896631176);
    assert_eq!(dd.gamma, 0.2026699815455325);
}

#[test]
pub fn rotate_vector() {
    let mut dd: DirectionCosine<f64> = DirectionCosine {
        alpha: 0.2140,
        beta: 0.8621,
        gamma: 0.7821,
    };
    dd.rotate_3d_vector(1.0.sin(), 1.0.cos(), 2.0.sin(), 2.0.cos());

    assert_eq!(dd.alpha, -1.0369691350703922);
    assert_eq!(dd.beta, 0.3496694784021821);
    assert_eq!(dd.gamma, 0.6407833194623658);
}

#[test]
pub fn trajectory() {
    let mut pp: MCParticle<f64> = MCParticle::default();
    // sets parameters
    let vv: MCVector<f64> = MCVector {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };
    let d_cos: DirectionCosine<f64> = DirectionCosine {
        alpha: 1.0 / 3.0.sqrt(),
        beta: 1.0 / 3.0.sqrt(),
        gamma: 1.0 / 3.0.sqrt(),
    };
    let e: f64 = 1.0;
    pp.velocity = vv;
    pp.direction_cosine = d_cos;
    pp.kinetic_energy = e;
    let mut seed: u64 = 90374384094798327;
    let energy = rng_sample(&mut seed);
    let angle = rng_sample(&mut seed);

    // update & print result
    update_trajectory(energy, angle, &mut pp);

    assert!((pp.direction_cosine.alpha - 0.620283).abs() < 1.0e-6);
    assert!((pp.direction_cosine.beta - 0.620283).abs() < 1.0e-6);
    assert!((pp.direction_cosine.gamma - (-0.480102)).abs() < 1.0e-6);
    assert!((pp.kinetic_energy - 0.398665).abs() < 1.0e-6);
}

#[test]
pub fn move_particle() {
    // copy pasting the core of the function to avoid the init of whole structures
    let move_factor: f64 = 0.5 * SMALL_FLOAT;
    let mut coord: MCVector<f64> = MCVector {
        x: 1.923,
        y: -2.45,
        z: 5.013,
    };
    let move_to: MCVector<f64> = MCVector {
        x: 4.0,
        y: 0.241,
        z: 7.9020,
    };

    coord += (move_to - coord) * move_factor;

    assert!(coord.is_almost_equal(&MCVector {
        x: 1.92300000010385,
        y: -2.44999999986545,
        z: 5.01300000014445,
    }));
}

#[test]
pub fn compute_volume() {
    let v0: MCVector<f64> = MCVector {
        x: 1.923,
        y: -2.45,
        z: 5.013,
    };
    let v1: MCVector<f64> = MCVector {
        x: 3.041,
        y: 1.368,
        z: 9.143,
    };
    let v2: MCVector<f64> = MCVector {
        x: 6.235,
        y: 0.325,
        z: 2.502,
    };
    let v3: MCVector<f64> = MCVector {
        x: 1.634,
        y: -1.34,
        z: 3.873,
    };

    let tmp0 = v0 - v3;
    let tmp1 = v1 - v3;
    let tmp2 = v2 - v3;

    let volume = tmp0.dot(&tmp1.cross(&tmp2));

    assert_eq!(volume, -44.197674792000015);
}

#[test]
pub fn macros() {
    // init
    let v0: MCVector<f64> = MCVector {
        x: 1.923,
        y: -2.45,
        z: 5.013,
    };
    let v1: MCVector<f64> = MCVector {
        x: 3.041,
        y: 1.368,
        z: 9.143,
    };
    let v2: MCVector<f64> = MCVector {
        x: 6.235,
        y: 0.325,
        z: 2.502,
    };
    let facet_coords = [v0, v1, v2];
    let intersection_pt: MCVector<f64> = MCVector {
        x: 1.634,
        y: -1.34,
        z: 3.873,
    };
    let bounding_box_tolerance: f64 = 1.0e-9;

    // if the point doesn't belong to the facet, returns huge_f
    macro_rules! belongs_or_return {
        ($axis: ident, $res: ident) => {
            let below: bool = (facet_coords[0].$axis
                > intersection_pt.$axis + bounding_box_tolerance)
                & (facet_coords[1].$axis > intersection_pt.$axis + bounding_box_tolerance)
                & (facet_coords[2].$axis > intersection_pt.$axis + bounding_box_tolerance);
            let above: bool = (facet_coords[0].$axis
                < intersection_pt.$axis - bounding_box_tolerance)
                & (facet_coords[1].$axis < intersection_pt.$axis - bounding_box_tolerance)
                & (facet_coords[2].$axis < intersection_pt.$axis - bounding_box_tolerance);

            let $res = (below, above); // changed this part for easier testing
        };
    }

    // scalar value of the cross product between AB & AC
    macro_rules! ab_cross_ac {
        ($ax: expr, $ay: expr, $bx: expr, $by: expr, $cx: expr, $cy: expr) => {
            ($bx - $ax) * ($cy - $ay) - ($by - $ay) * ($cx - $ax)
        };
    }

    belongs_or_return!(x, belong_x);
    belongs_or_return!(y, belong_y);
    belongs_or_return!(z, belong_z);

    let cross1 = ab_cross_ac!(
        facet_coords[0].x,
        facet_coords[0].y,
        facet_coords[1].x,
        facet_coords[1].y,
        intersection_pt.x,
        intersection_pt.y
    );
    let cross2 = ab_cross_ac!(
        facet_coords[1].x,
        facet_coords[1].y,
        facet_coords[2].x,
        facet_coords[2].y,
        intersection_pt.x,
        intersection_pt.y
    );
    let cross0 = ab_cross_ac!(
        facet_coords[2].x,
        facet_coords[2].y,
        facet_coords[0].x,
        facet_coords[0].y,
        intersection_pt.x,
        intersection_pt.y
    );

    assert!(belong_x.0 | belong_x.1);
    assert!(!(belong_y.0 | belong_y.1));
    assert!(!(belong_z.0 | belong_z.1));
    assert_eq!(cross0, -5.588295000000003);
    assert_eq!(cross1, 2.3443820000000004);
    assert_eq!(cross2, -10.116853000000003);
}
