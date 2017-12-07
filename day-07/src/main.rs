extern crate regex;

#[cfg(not(test))]
const INPUT: &'static str = include_str!("input.txt");

mod tower;

#[cfg(not(test))]
fn main() {
    println!("Hello World");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read_tower() {
        tower::read_tower("
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
    }
}
