#[cfg(not(test))]
fn main() {
    const INPUT: &str = include_str!("input.txt");

    let firewall = make_firewall(INPUT);

    println!("Part 1: {}", trip_severity(&firewall, 0).1);
    println!("Part 2: {}", workout_firewall_delay(&firewall));
}

type Firewall = Vec<(u32, u32)>;

/// Converts the input into a firewall
fn make_firewall(input: &str) -> Firewall {
    input.trim()
        .lines()
        .map(| line | {
            let parts: Vec<u32> =
                line.split(": ")
                    .map( | part | part.trim().parse().expect("Expected Int"))
                    .collect();

            assert_eq!(parts.len(), 2, "Expected two numbers from firewall line");
            assert!(parts[1] > 1, "Firewall depth must be greater than 1");

            (parts[0], parts[1])
        })
    .collect()
}

/// Calculates a trip severity
fn trip_severity(firewall: &Firewall, delay: u32) -> (bool, u32) {
    firewall.iter().fold((false, 0), | (caught, accum), layer | {
        let (index, depth) = *layer;

        // Depth = 5
        // Cycle = 0 1 2 3 4 [3 2 1]
        // Cycle Length = 8
        let cycle_length = (depth * 2) - 2;

        let cycle_pos = (index + delay) % cycle_length;

        if cycle_pos == 0 {
            (true, accum + (index * depth))
        } else {
            (caught, accum)
        }
    })
}

/// Works out how long to wait before entering the firewall such that we can clear it without
/// being detected
fn workout_firewall_delay(firewall: &Firewall) -> u32 {
    let mut delay = 0;

    while trip_severity(firewall, delay).0 {
        delay = delay + 1
    }

    delay
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        const INPUT: &str = "
            0: 3
            1: 2
            4: 4
            6: 4
        ";

        let firewall = make_firewall(INPUT);

        assert_eq!(
            trip_severity(&firewall, 0).1,
            (0 * 3) + (6 * 4),
            "Trip severity is wrong"
        );

        assert_eq!(
            workout_firewall_delay(&firewall),
            10,
            "Initial Delay is wrong"
        )
    }
}
