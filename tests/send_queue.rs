use fastiron::send_queue::{SendQueue, SendQueueTuple};

#[test]
fn reserve() {
    let tt = SendQueueTuple {
        neighbor: 0,
        particle_index: 0,
    };
    let mut queue = SendQueue { data: vec![tt; 10] };

    assert_eq!(queue.size(), 10);
    queue.reserve(20);
    assert_eq!(queue.data.capacity(), 20);
}

#[test]
fn neighbor_size() {
    let t0 = SendQueueTuple {
        neighbor: 0,
        particle_index: 0,
    };
    let t1 = SendQueueTuple {
        neighbor: 1,
        particle_index: 1,
    };
    let t2 = SendQueueTuple {
        neighbor: 1,
        particle_index: 2,
    };
    let t3 = SendQueueTuple {
        neighbor: 4,
        particle_index: 3,
    };
    let t4 = SendQueueTuple {
        neighbor: 3,
        particle_index: 4,
    };
    let t5 = SendQueueTuple {
        neighbor: 6,
        particle_index: 5,
    };
    let t6 = SendQueueTuple {
        neighbor: 1,
        particle_index: 6,
    };
    let t7 = SendQueueTuple {
        neighbor: 3,
        particle_index: 7,
    };
    let t8 = SendQueueTuple {
        neighbor: 0,
        particle_index: 8,
    };
    let t9 = SendQueueTuple {
        neighbor: 5,
        particle_index: 9,
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
    let t0 = SendQueueTuple {
        neighbor: 0,
        particle_index: 0,
    };
    let t1 = SendQueueTuple {
        neighbor: 1,
        particle_index: 1,
    };
    let t2 = SendQueueTuple {
        neighbor: 1,
        particle_index: 2,
    };
    let t3 = SendQueueTuple {
        neighbor: 4,
        particle_index: 3,
    };
    let t4 = SendQueueTuple {
        neighbor: 3,
        particle_index: 4,
    };

    let mut queue = SendQueue {
        data: vec![t0, t1, t2, t3],
    };
    queue.push(3, 4);

    assert_eq!(queue.size(), 5);
    assert_eq!(queue.data[queue.size() - 1], t4);

    queue.clear();

    assert_eq!(queue.size(), 0);
}
