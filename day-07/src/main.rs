extern crate regex;

#[cfg(not(test))]
const INPUT: &'static str = include_str!("input.txt");

mod tower;

#[cfg(not(test))]
fn main() {
    let bottom = tower::read_tower(INPUT);

    println!("Part 1: {}", bottom.name);

    let change = bottom.required_change_to_balance();
    println!("Part 2: {:?}", change);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read_tower() {
        let bottom = tower::read_tower("
            pbga (66)
            xhth (57)
            ebii (61)
            havc (66)
            ktlj (57)
            fwft (72) -> ktlj, cntj, xhth
            qoyq (66)
            padx (45) -> pbga, havc, qoyq
            tknk (41) -> ugml, padx, fwft
            jptl (61)
            ugml (68) -> gyxo, ebii, jptl
            gyxo (61)
            cntj (57)
        ");

        assert_eq!(bottom.name, "tknk");

        println!("Part 2");
        bottom.print();
        assert_eq!(bottom.required_change_to_balance(), Some(60));
    }
}
