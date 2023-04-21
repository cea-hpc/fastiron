// Tests used to compare the results of certain computation heavy
// functions with their result in the original code.
// Results are hard coded.

use fastiron::data::mc_vector::MCVector;

#[test]
fn compute_volume() {
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
fn macros() {
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
