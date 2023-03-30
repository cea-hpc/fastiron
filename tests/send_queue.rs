use fastiron::{
    data::send_queue::{SendQueue, SendQueueTuple},
    particles::mc_particle::MCParticle,
};

#[test]
fn reserve() {
    let tt: SendQueueTuple<f64> = SendQueueTuple {
        neighbor: 0,
        particle: MCParticle::default(),
    };
    let mut queue = SendQueue { data: vec![tt; 10] };

    assert_eq!(queue.size(), 10);
    queue.reserve(20);
    assert_eq!(queue.data.capacity(), 20);
}

#[test]
fn neighbor_size() {
    let t0: SendQueueTuple<f64> = SendQueueTuple {
        neighbor: 0,
        particle: MCParticle::default(),
    };
    let t1: SendQueueTuple<f64> = SendQueueTuple {
        neighbor: 1,
        particle: MCParticle::default(),
    };
    let t2: SendQueueTuple<f64> = SendQueueTuple {
        neighbor: 1,
        particle: MCParticle::default(),
    };
    let t3: SendQueueTuple<f64> = SendQueueTuple {
        neighbor: 4,
        particle: MCParticle::default(),
    };
    let t4: SendQueueTuple<f64> = SendQueueTuple {
        neighbor: 3,
        particle: MCParticle::default(),
    };
    let t5: SendQueueTuple<f64> = SendQueueTuple {
        neighbor: 6,
        particle: MCParticle::default(),
    };
    let t6: SendQueueTuple<f64> = SendQueueTuple {
        neighbor: 1,
        particle: MCParticle::default(),
    };
    let t7: SendQueueTuple<f64> = SendQueueTuple {
        neighbor: 3,
        particle: MCParticle::default(),
    };
    let t8: SendQueueTuple<f64> = SendQueueTuple {
        neighbor: 0,
        particle: MCParticle::default(),
    };
    let t9: SendQueueTuple<f64> = SendQueueTuple {
        neighbor: 5,
        particle: MCParticle::default(),
    };

    let queue = SendQueue {
        data: vec![t0, t1, t2, t3, t4, t5, t6, t7, t8, t9],
    };

    assert_eq!(queue.neighbor_size(0), 2);
    assert_eq!(queue.neighbor_size(1), 3);
    assert_eq!(queue.neighbor_size(2), 0);
    assert_eq!(queue.neighbor_size(3), 2);
    assert_eq!(queue.neighbor_size(4), 1);
    assert_eq!(queue.neighbor_size(5), 1);
    assert_eq!(queue.neighbor_size(6), 1);
    assert_eq!(queue.neighbor_size(7), 0);
}

#[test]
fn push_get_clear() {
    let t0: SendQueueTuple<f64> = SendQueueTuple {
        neighbor: 0,
        particle: MCParticle::default(),
    };
    let t1: SendQueueTuple<f64> = SendQueueTuple {
        neighbor: 1,
        particle: MCParticle::default(),
    };
    let t2: SendQueueTuple<f64> = SendQueueTuple {
        neighbor: 1,
        particle: MCParticle::default(),
    };
    let t3: SendQueueTuple<f64> = SendQueueTuple {
        neighbor: 4,
        particle: MCParticle::default(),
    };
    let t4: SendQueueTuple<f64> = SendQueueTuple {
        neighbor: 3,
        particle: MCParticle::default(),
    };

    let mut queue = SendQueue {
        data: vec![t0, t1, t2, t3],
    };
    queue.push(3, &MCParticle::default());

    assert_eq!(queue.size(), 5);
    assert_eq!(queue.data[queue.size() - 1], t4);

    queue.clear();

    assert_eq!(queue.size(), 0);
}
