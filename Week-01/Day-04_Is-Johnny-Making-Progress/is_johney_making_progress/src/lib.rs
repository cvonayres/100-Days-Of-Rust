pub fn progress_days(record: Vec<i32>) -> usize {
    let mut qty = 0;
    let mut last = record[0];

    for day in record {
        if day > last {
            qty += 1;
        }
        last = day;
    }
    qty
}

pub fn progress_days_alt(record: Vec<i32>) -> usize {
    record.windows(2).filter(|window| window[1] > window[0]).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_old() {
        let test = vec![3, 4, 1, 2];
        assert_eq!(progress_days(test), 2);

        let test = vec![10, 11, 12, 9, 10];
        assert_eq!(progress_days(test), 3);

        let test = vec![6, 5, 4, 3, 2, 9];
        assert_eq!(progress_days(test), 1);

        let test = vec![9, 9];
        assert_eq!(progress_days(test), 0);
    }

    #[test]
    fn example_new() {
        let test = vec![3, 4, 1, 2];
        assert_eq!(progress_days_alt(test), 2);

        let test = vec![10, 11, 12, 9, 10];
        assert_eq!(progress_days_alt(test), 3);

        let test = vec![6, 5, 4, 3, 2, 9];
        assert_eq!(progress_days_alt(test), 1);

        let test = vec![9, 9];
        assert_eq!(progress_days_alt(test), 0);
    }
}
