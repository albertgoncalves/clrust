#[cfg(test)]
mod test {
    #[test]
    fn distance() {
        macro_rules! assert_distance {
            ($ax:expr, $ay:expr, $bx:expr, $by:expr, $d:expr) => {
                let a: crate::Point = crate::Point { x: $ax, y: $ay };
                let b: crate::Point = crate::Point { x: $bx, y: $by };
                assert_eq!(crate::distance(&a, &b), $d)
            };
        }
        assert_distance!(0.0, 0.0, 3.0, 4.0, 5.0);
        assert_distance!(3.0, 4.0, 0.0, 0.0, 5.0);
        assert_distance!(0.0, 0.0, 5.0, 12.0, 13.0);
        assert_distance!(5.0, 12.0, 0.0, 0.0, 13.0);
    }

    #[test]
    fn row_to_point() {
        macro_rules! assert_row_to_point {
            ($s:expr, $n:expr, $cx:expr, $cy:expr, $x:expr, $y:expr) => {
                assert_eq!(
                    crate::row_to_point($s, $n, $cx, $cy),
                    Some(crate::Point { x: $x, y: $y }),
                );
            };
            ($s:expr, $n:expr, $cx:expr, $cy:expr) => {
                assert_eq!(crate::row_to_point($s, $n, $cx, $cy), None);
            };
        }
        assert_row_to_point!("0,1", 2, 0, 1, 0.0, 1.0);
        assert_row_to_point!("0,1,2,3", 4, 1, 3, 1.0, 3.0);
        assert_row_to_point!("0,1,2,3", 5, 1, 3);
        assert_row_to_point!("0,1,2,3", 5, 1, 3);
        assert_row_to_point!("0,1,2,3", 4, 0, 0);
        assert_row_to_point!("0,1,2,3", 4, 1, 4);
    }
}
