use fastiron::{parameters::{GeometryParameters, Shape}, mc::{mc_vector::MCVector, mc_domain::MCDomain}, constants::CustomFloat};

#[test]
pub fn inside_material() {
    // Sphere centered at (2.0, 2.0, 2.0), radius 1.0
    let geom_a = GeometryParameters { 
        material_name: String::from("mat_a"),
        shape: Shape::Sphere, 
        radius: 1.0, 
        x_center: 2.0, 
        y_center: 2.0, 
        z_center: 2.0, 
        ..Default::default() 
    };
    let geom_b = GeometryParameters {
        material_name: String::from("mat_b"),
        shape: Shape::Brick,
        x_min: 0.0,
        y_min: 0.0,
        z_min: 0.0,
        x_max: 4.0,
        y_max: 2.0,
        z_max: 1.0,
        ..Default::default()
    };
    let geom_c = GeometryParameters { 
        material_name: String::from("mat_c"),
        shape: Shape::Sphere, 
        radius: 2.0, 
        x_center: 6.0, 
        y_center: 2.0, 
        z_center: 2.0, 
        ..Default::default() 
    };
    let geom_d = GeometryParameters {
        material_name: String::from("mat_d"),
        shape: Shape::Brick,
        x_min: 0.0,
        y_min: 2.0,
        z_min: 0.0,
        x_max: 4.0,
        y_max: 4.0,
        z_max: 1.0,
        ..Default::default()
    };

    // is_inside
    let r1 = MCVector {x: 2.0, y: 1.0, z: 0.5 }; // in brick b
    assert!(MCDomain::is_inside(&geom_b, &r1));

    let r2 = MCVector {x: 2.0, y: 2.1, z: 0.5 }; // out brick b, in brick d
    assert!(!MCDomain::is_inside(&geom_b, &r2));

    let r3 = MCVector {x: 1.5, y: 2.0, z: 2.5 }; // in sphere a
    assert!(MCDomain::is_inside(&geom_a, &r3));

    let r4 = MCVector {x: 2.0, y: 2.0, z: 3.1 }; // out sphere a
    assert!(!MCDomain::is_inside(&geom_a, &r4));

    let r5 = MCVector {x: 2.0, y: 2.0, z: 1.0 }; // in both a, b (single common point)
    assert!(MCDomain::is_inside(&geom_a, &r5));
    assert!(MCDomain::is_inside(&geom_b, &r5));

    let r6 = MCVector {x: 5.0, y: 2.0, z: 2.0 }; // out both a, b, in c
    assert!(!MCDomain::is_inside(&geom_b, &r6));
    assert!(!MCDomain::is_inside(&geom_b, &r6));

    // find_material
    let geoms = vec![geom_a, geom_b, geom_c, geom_d];
    assert_eq!(MCDomain::find_material(&geoms, &r1), String::from("mat_b"));
    assert_eq!(MCDomain::find_material(&geoms, &r2), String::from("mat_d"));
    assert_eq!(MCDomain::find_material(&geoms, &r3), String::from("mat_a"));
    assert_eq!(MCDomain::find_material(&geoms, &r5), String::from("mat_a")); // first of the list takes priority
    assert_eq!(MCDomain::find_material(&geoms, &r6), String::from("mat_c")); 

}

