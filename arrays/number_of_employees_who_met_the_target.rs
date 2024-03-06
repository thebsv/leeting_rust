pub fn number_of_employees_who_met_target(hours: Vec<i32>, target: i32) -> i32 {
    hours.iter().filter(|&n| *n >= target).count() as i32
}

pub fn main() {
    let hours = vec![0, 1, 2, 3, 4];
    let target = 2;
    println!(
        "number: {}",
        number_of_employees_who_met_target(hours, target)
    );
}
