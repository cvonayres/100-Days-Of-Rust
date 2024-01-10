/// Returns tuple for (# vegetarian skewers, # non-vegetarian skewers)
pub fn no_of_skewer(bbq: &Vec<&str>) -> (usize, usize) {
    let meat_count = bbq.iter().filter(|&skewer| skewer.contains('x')).count();
    let veg_count = bbq.len() - meat_count;

    (veg_count, meat_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let test_1 = vec![
            "--xo--x--ox--",
            "--xx--x--xx--",
            "--oo--o--oo--",
            "--xx--x--ox--",
            "--xx--x--ox--",
        ];
        let test_2 = vec![
            "--oooo-ooo--",
            "--xx--x--xx--",
            "--o---o--oo--",
            "--xx--x--ox--",
            "--xx--x--ox--",
        ];
        let test_3 = vec![
            "--oooo-ooo--",
            "--xxxxxxxx--",
            "--o---",
            "-o-----o---x--",
            "--o---o-----",
        ];
        assert_eq!(no_of_skewer(&test_1), (1, 4));
        assert_eq!(no_of_skewer(&test_2), (2, 3));
        assert_eq!(no_of_skewer(&test_3), (3, 2));
    }
}
