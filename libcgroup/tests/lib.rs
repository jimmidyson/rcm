extern crate libcgroup;
use libcgroup::*;

#[test]
#[cfg(feature = "sudo")]
fn test_manipulate_cgroup() {
    let cg = match CGroup::new("/rust-cgroup-test") {
        Ok(cgroup) => cgroup,
        Err(err) => panic!("Should not have returned error: {}", err.description),
    };

    let ctrlr = match cg.add_controller("memory") {
        Ok(c) => c,
        Err(err) => panic!("{}", err.description),
    };

    match ctrlr.add_value_uint64("memory.limit_in_bytes", 409666u64) {
        Ok(c) => c,
        Err(err) => println!("{}", err.description),
    };

    match cg.create() {
        Ok(m) => m,
        Err(err) => panic!("{}", err.description),
    };

    let lim = match ctrlr.get_value_uint64("memory.limit_in_bytes") {
        Ok(m) => m,
        Err(err) => panic!("{}", err.description),
    };
    println!("{}", lim);
    assert!(lim == 409666u64);

    match cg.delete() {
        Ok(c) => c,
        Err(err) => panic!("{}", err.description),
    };
}

#[test]
fn test_get_values() {
    let cg = match CGroup::new("/") {
        Ok(cgroup) => cgroup,
        Err(err) => panic!("Should not have returned error: {}", err.description),
    };
    match cg.get() {
        Ok(_) => (),
        Err(err) => panic!("Should not have returned error: {}", err.description),
    };

    let ctrlr = match cg.get_controller("memory") {
        Ok(c) => c,
        Err(err) => panic!("{}", err.description),
    };

    let lim = match ctrlr.get_value_int64("memory.limit_in_bytes") {
        Ok(m) => m,
        Err(err) => panic!("{}", err.description),
    };
    assert!(lim > 0);
    let ulim = match ctrlr.get_value_uint64("memory.limit_in_bytes") {
        Ok(m) => m,
        Err(err) => panic!("{}", err.description),
    };
    assert!(ulim > 0);
    let h = match ctrlr.get_value_bool("memory.use_hierarchy") {
        Ok(m) => m,
        Err(err) => panic!("{}", err.description),
    };
    assert!(h);
    let slim = match ctrlr.get_value_string("memory.limit_in_bytes") {
        Ok(m) => m,
        Err(err) => panic!("{}", err.description),
    };
    assert!(slim.len() > 0);
}

#[test]
fn iterate_cgroup_mount_points() {
    let mut found = false;
    for c in cgroup_mount_points_iter() {
        found = match c {
            Ok(_) => true,
            Err(err) => panic!("{}", err.description),
        }
    }
    assert!(found);
}

#[test]
fn test_cgroup_mount_points() {
    let v = match cgroup_mount_points() {
        Ok(mps) => mps,
        Err(err) => panic!("{}", err.description),
    };
    assert!(v.len() > 0);
}

#[test]
fn iterate_cgroup_tree() {
    let mut found = false;
    for c in cgroup_walk_tree_iter("memory") {
        found = match c {
            Ok(fi) => {
                println!("{:?}", fi);
                true
            }
            Err(err) => panic!("{}", err.description),
        }
    }
    assert!(found);
}
