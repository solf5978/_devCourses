use std::collections::{HashMap, HashSet};

fn vectors() {
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);

    println!("a = {:?}", a);

    a.push(44);
    println!("a = {:?}", a);
    println!("a[0] = {}", a[0]);

    let idx: usize = 0;
    a[idx] = 312;
    println!("a[0] = {}", a[idx]);

    match a.get(6) {
        Some(x) => println!("a[6] = {}", x),
        None => println!("error tho"),
    }

    for x in &a { println!("{}", x); }

    if let Some(x) = a.pop() {
        println!("{}", x);
    } // while let works too
}

fn create_hashmap() {
    let mut shapes = HashMap::new();
    shapes.insert(String::from("triangle"), 3);
    shapes.insert(String::from("square"), 4);

    println!("a square has {} sides", shapes["square".into()]);

    for (k, v) in &shapes {
        println!("{} : {}", k, v);
    }

    println!("{:?}", shapes);

    shapes.entry("circle".into()).or_insert(1);
    {
        let actual = shapes.entry("circle".into()).or_insert(1);
        *actual = 0;
    }

    println!("{:?}", shapes);
}

fn create_hashset() {
    let mut greeks = HashSet::new();
    greeks.insert("gamma");
    greeks.insert("delta");
    println!("{:?}", greeks);

    greeks.insert("delta"); //no changes
    println!("{:?}", greeks);
    let added_vega = greeks.insert("vega");

    if added_vega {
        println!("Yes it is in");
    }

    if !greeks.contains("kappa") {
        println!("we dont have this");
    }

    let removed = greeks.remove("delta");
    if removed {
        println!("removed!!!")
    }

    let _1_5: HashSet<_> = (1..=5).collect();
    let _11_15: HashSet<_> = (11..=15).collect();
    let _21_25: HashSet<_> = (21..=25).collect();
    let _31_35: HashSet<_> = (31..=35).collect();

    println!(
        "is {:?} a subset of {:?}? {}", _1_5, _11_15, _1_5.is_subset(&_11_15)
    );

    println!(
        "is {:?} a subset of {:?}? {}", _1_5, _11_15, _1_5.is_disjoint(&_11_15)
    );
}

fn main() {
    vectors();
    create_hashmap();
    create_hashset();
}
