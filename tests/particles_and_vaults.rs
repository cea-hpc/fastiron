use fastiron::{mc::mc_base_particle::MCBaseParticle, particle_vault::ParticleVault};

#[test]
fn collapse_enough_space() {
    // 2 vaults of 8 particles each
    let p1: Option<MCBaseParticle<f64>> = Some(MCBaseParticle::default());
    let p2: Option<MCBaseParticle<f64>> = Some(MCBaseParticle::default());
    let p3: Option<MCBaseParticle<f64>> = Some(MCBaseParticle::default());
    let p4: Option<MCBaseParticle<f64>> = Some(MCBaseParticle::default());
    let p5: Option<MCBaseParticle<f64>> = Some(MCBaseParticle::default());
    let p6: Option<MCBaseParticle<f64>> = Some(MCBaseParticle::default());
    let p7: Option<MCBaseParticle<f64>> = Some(MCBaseParticle::default());
    let p8: Option<MCBaseParticle<f64>> = Some(MCBaseParticle::default());
    let mut vault1: ParticleVault<f64> = ParticleVault { particles: vec![p1, p2, p3, p4, p5, p6, p7, p8] };
    let p9: Option<MCBaseParticle<f64>> = Some(MCBaseParticle::default());
    let p10: Option<MCBaseParticle<f64>> = Some(MCBaseParticle::default());
    let p11: Option<MCBaseParticle<f64>> = Some(MCBaseParticle::default());
    let p12: Option<MCBaseParticle<f64>> = Some(MCBaseParticle::default());
    let p13: Option<MCBaseParticle<f64>> = Some(MCBaseParticle::default());
    let p14: Option<MCBaseParticle<f64>> = Some(MCBaseParticle::default());
    let p15: Option<MCBaseParticle<f64>> = Some(MCBaseParticle::default());
    let p16: Option<MCBaseParticle<f64>> = Some(MCBaseParticle::default());
    let mut vault2: ParticleVault<f64> = ParticleVault { particles: vec![p9, p10, p11, p12, p13, p14, p15, p16] };
    
    // max size for vault is 20
    vault1.collapse(20, &mut vault2);
    // all particles must have been transfered to the first vault
    assert_eq!(vault1.size(), 16);
    assert_eq!(vault2.size(), 0);
}

#[test]
fn collapse_missing_space() {
    // 2 vaults of 8 particles each
    let p1: Option<MCBaseParticle<f64>> = Some(MCBaseParticle::default());
    let p2: Option<MCBaseParticle<f64>> = Some(MCBaseParticle::default());
    let p3: Option<MCBaseParticle<f64>> = Some(MCBaseParticle::default());
    let p4: Option<MCBaseParticle<f64>> = Some(MCBaseParticle::default());
    let p5: Option<MCBaseParticle<f64>> = Some(MCBaseParticle::default());
    let p6: Option<MCBaseParticle<f64>> = Some(MCBaseParticle::default());
    let p7: Option<MCBaseParticle<f64>> = Some(MCBaseParticle::default());
    let p8: Option<MCBaseParticle<f64>> = Some(MCBaseParticle::default());
    let mut vault1: ParticleVault<f64> = ParticleVault { particles: vec![p1, p2, p3, p4, p5, p6, p7, p8] };
    let p9: Option<MCBaseParticle<f64>> = Some(MCBaseParticle::default());
    let p10: Option<MCBaseParticle<f64>> = Some(MCBaseParticle::default());
    let p11: Option<MCBaseParticle<f64>> = Some(MCBaseParticle::default());
    let p12: Option<MCBaseParticle<f64>> = Some(MCBaseParticle::default());
    let p13: Option<MCBaseParticle<f64>> = Some(MCBaseParticle::default());
    let p14: Option<MCBaseParticle<f64>> = Some(MCBaseParticle::default());
    let p15: Option<MCBaseParticle<f64>> = Some(MCBaseParticle::default());
    let p16: Option<MCBaseParticle<f64>> = Some(MCBaseParticle::default());
    let mut vault2: ParticleVault<f64> = ParticleVault { particles: vec![p9, p10, p11, p12, p13, p14, p15, p16] };

    // max size for vault is 10
    vault1.collapse(10, &mut vault2);
    // vault 1 should be full (10), and vault2 should have the leftovers (6)
    assert_eq!(vault1.size(), 10);
    assert_eq!(vault2.size(), 6);
}

#[test]
fn collapse_dummy_test() {
    let mut v1: Vec<u32> = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let mut v2: Vec<u32> = vec![9, 10, 11, 12, 13, 14, 15, 16];

    collapse_dummy(&mut v1, 10, &mut v2);
    println!("v1: {v1:?}");
    println!("v2: {v2:?}");
    assert_eq!(v1.len(), 10);
    assert_eq!(v2.len(), 6);
}

fn collapse_dummy(v1: &mut Vec<u32>, fill_size: usize, v2: &mut Vec<u32>) {
    if v1.len() + v2.len() < fill_size {
        v1.append(v2);
    } else {
        v1.extend_from_slice(&v2[..fill_size-v1.len()]);
        *v2 = Vec::from(&v2[fill_size-v1.len()..]);
    }
}