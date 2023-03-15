use fastiron::{
    mc::{mc_base_particle::MCBaseParticle, mc_particle::MCParticle},
    particle_vault::ParticleVault,
};

#[test]
fn append_pop_clear() {
    // 2 vaults of 8 particles each
    let pp: Option<MCBaseParticle<f64>> = Some(MCBaseParticle::default());
    let mut vault1: ParticleVault<f64> = ParticleVault {
        particles: vec![pp.clone(); 8],
    };
    let mut vault2: ParticleVault<f64> = ParticleVault {
        particles: vec![pp; 8],
    };

    vault1.append(&vault2);
    assert_eq!(vault1.size(), 16);
    assert_eq!(vault2.size(), 8);
    _ = vault1.pop_particle();
    vault2.clear();
    assert_eq!(vault1.size(), 15);
    assert_eq!(vault2.size(), 0);
    assert!(vault2.pop_particle().is_none())
}

#[test]
fn collapse_enough_space() {
    // 2 vaults of 8 particles each
    let pp: Option<MCBaseParticle<f64>> = Some(MCBaseParticle::default());
    let mut vault1: ParticleVault<f64> = ParticleVault {
        particles: vec![pp.clone(); 8],
    };
    let mut vault2: ParticleVault<f64> = ParticleVault {
        particles: vec![pp; 8],
    };

    // max size for vault is 20 so fill_size is 12
    vault1.collapse(12, &mut vault2);
    // all particles must have been transfered to the first vault
    assert_eq!(vault1.size(), 16);
    assert_eq!(vault2.size(), 0);
}

#[test]
fn collapse_missing_space() {
    // 2 vaults of 8 particles each
    let pp: Option<MCBaseParticle<f64>> = Some(MCBaseParticle::default());
    let mut vault1: ParticleVault<f64> = ParticleVault {
        particles: vec![pp.clone(); 8],
    };
    let mut vault2: ParticleVault<f64> = ParticleVault {
        particles: vec![pp; 8],
    };

    // max size for vault is 10 so fil_size is 2
    vault1.collapse(2, &mut vault2);
    // vault 1 should be full (10), and vault2 should have the leftovers (6)
    assert_eq!(vault1.size(), 10);
    assert_eq!(vault2.size(), 6);
}

#[test]
fn reserve() {
    let pp: Option<MCBaseParticle<f64>> = Some(MCBaseParticle::default());
    let mut vault1: ParticleVault<f64> = ParticleVault {
        particles: vec![pp; 8],
    };
    assert_eq!(vault1.size(), 8);
    vault1.reserve(20);
    // capacity should be 20, not 28
    assert_eq!(vault1.particles.capacity(), 20);
}

#[test]
fn put_invalidate() {
    let pp: Option<MCBaseParticle<f64>> = Some(MCBaseParticle::default());
    let mut vault1: ParticleVault<f64> = ParticleVault {
        particles: vec![pp; 8],
    };

    vault1.invalidate_particle(0);
    vault1.invalidate_particle(4);
    vault1.invalidate_particle(7);
    assert!(vault1.get_base_particle(0).is_none());
    assert!(vault1.get_base_particle(4).is_none());
    assert!(vault1.get_base_particle(7).is_none());

    vault1.put_particle(MCParticle::default(), 4);
    assert!(vault1.get_particle(4).is_some());
}

#[test]
fn erase_swap_particles() {
    let p1: Option<MCBaseParticle<f64>> = Some(MCBaseParticle {
        identifier: 103,
        ..Default::default()
    });
    let p2: Option<MCBaseParticle<f64>> = Some(MCBaseParticle {
        identifier: 406,
        ..Default::default()
    });
    let mut vault = ParticleVault {
        particles: vec![None, None, p1, None, None, None, p2],
    };
    // vault is size 2, capacity 7
    assert_eq!(vault.size(), 2);
    assert_eq!(vault.particles.len(), 7);

    println!("before: {vault:#?}");

    vault.erase_swap_particles(2); // p1 and p2 switch; p1 is popped into oblivion

    println!("after: {vault:#?}");

    // vault should be size 1, capacity still 7
    assert_eq!(vault.size(), 1);
    assert_eq!(vault.particles.len(), 7);
    // At index 2 should be p2
    assert_eq!(vault.get_base_particle(2).unwrap().identifier, 406);
    // If we pop an element, we should have None since p1 was deleted
    assert!(vault.particles.last().unwrap().is_none());
}
