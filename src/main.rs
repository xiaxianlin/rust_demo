#![allow(dead_code)]
#![allow(unused_variables)]

mod advance;
mod demo;
mod learn;
mod minigrep;

enum Gender {
    Male,
    Female,
}

fn bmr(gender: Gender, age: f32, weight: f32, height: f32) -> (f32, f32) {
    let bmr1 = match gender {
        Gender::Male => 88.362 + (13.397 * weight) + (4.799 * height) - (5.677 * age),
        Gender::Female => 447.593 + (9.247 * weight) + (3.098 * height) - (4.330 * age),
    };

    let bmr2 = match gender {
        Gender::Male => (10.0 * weight) + (6.25 * height) - (5.0 * age) + 5.0,
        Gender::Female => (10.0 * weight) + (6.25 * height) - (5.0 * age) - 161.0,
    };

    (bmr1, bmr2)
}

fn main() {
    // advance::async_demo::test();

    let v = bmr(Gender::Female, 30.0, 37.5, 158.0);
    println!("bmr: {:?}", v);

    let v = bmr(Gender::Male, 34.0, 90.8, 170.0);
    println!("bmr: {:?}", v);
}
