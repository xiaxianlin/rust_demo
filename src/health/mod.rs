pub enum Gender {
    Male,
    Female,
}

pub fn bmr(gender: Gender, age: f32, weight: f32, height: f32) -> (f32, f32) {
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
