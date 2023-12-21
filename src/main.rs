fn main() {
    let input = include_str!("../data/input.txt");
    let input: Vec<_> = input
        .split('\n')
        .map(|r| r.parse::<usize>().unwrap())
        .collect();

    let fuel: usize = input.iter().map(fuel_for_mass).sum();
    println!("[Part 1]");
    println!("Fuel required: {}", fuel);
}

fn fuel_for_mass(mass: &usize) -> usize {
    (mass / 3) - 2
}
