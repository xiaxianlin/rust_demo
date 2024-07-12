use vec::Vec;

mod vec;

fn main() {
    let mut v: Vec<u8> = vec::Vec::new();

    v.push(1);
    assert_eq!(v.size(), 1);

    let n = v.pop();
    assert_eq!(v.size(), 0);
    assert_eq!(n, Some(1));

    v.push(2);

    println!("v[0] = {:?}", v[0]);
    println!("v.first() = {:?}", v.first());

    v.push(3);
    v.push(4);
    v.insert(2, 5);
    println!("v[2] = {:?}", v[2]);

    let n = v.remove(2);
    assert_eq!(n, 5);
    println!("v[2] = {:?}", v[2]);

    // println!("v = {:?}", v);
}
