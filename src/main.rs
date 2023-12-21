fn main() {
    let input = include_str!("../data/input.txt");
    let input: Vec<_> = input
        .split('\n')
        .map(|r| r.parse::<usize>().unwrap())
        .collect();

    let fuel: usize = input.iter().map(fuel_for_mass).sum();
    println!("[Part 2]");
    println!("Fuel required: {}", fuel);
}

fn fuel_for_mass(mass: &usize) -> usize {
    let third = mass / 3;
    if third >= 2 {
        (third - 2) + fuel_for_mass(&(third - 2))
    } else {
        0
    }
}
