#[cfg(test)]
mod test {
    #[test]
    fn distance() {
        macro_rules! assert_distance {
            ($ax:expr, $ay:expr, $bx:expr, $by:expr, $d:expr) => {
                let a: crate::geom::Point =
                    crate::geom::Point { x: $ax, y: $ay, label: 0 };
                let b: crate::geom::Point =
                    crate::geom::Point { x: $bx, y: $by, label: 0 };
                assert_eq!(crate::geom::distance(&a, &b), $d)
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
                    Some(crate::geom::Point { x: $x, y: $y, label: 0 }),
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

    #[test]
    fn bounds() {
        assert_eq!(
            crate::geom::bounds(&[
                crate::geom::Point { x: 0.0, y: 1.0, label: 0 },
                crate::geom::Point { x: 9.0, y: -1.0, label: 0 },
                crate::geom::Point { x: -3.0, y: 10.0, label: 0 },
                crate::geom::Point { x: 12.0, y: 5.0, label: 0 },
            ]),
            crate::geom::Bounds {
                min_x: -3.0,
                max_x: 12.0,
                min_y: -1.0,
                max_y: 10.0,
            }
        );
    }
}