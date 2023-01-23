#[allow(dead_code)]
#[allow(unused_macros)]

/// # min!マクロ
macro_rules! min {
    ( $n:expr $(,)* ) => {{
        $n
    }};
    ( $n:expr, $m:expr $(,)* ) => {{
        ($n).min($m)
    }};
    ( $n:expr, $( $m:expr ),+ $(,)* ) => {{
        ($n).min( min!( $( $m ),+ ) )
    }};
}

/// # max!マクロ
macro_rules! max {
    ( $n:expr $(,)* ) => {{
        $n
    }};
    ( $n:expr, $m:expr $(,)* ) => {{
        ($n).max($m)
    }};
    ( $n:expr, $( $m:expr ),+ $(,)* ) => {{
        ($n).max( max!( $( $m ),+ ) )
    }};
}


#[cfg(test)]
mod test {
    
    #[test]
    fn test_min() {
        // usize
        assert_eq!(
            min!(1_usize, 6, 4, 2, 7, 0),
            0,
        );

        // isize
        assert_eq!(
            min!(6_isize, -7, -200, 4, 9000),
            -200,
        );

        // f64
        assert_eq!(
            min!(
                8.91_f64,
                92.0_f64,
                55.2_f64,
                1e100_f64,
            ),
            8.91
        );

        // &str
        assert_eq!(
            min!(
                "cat",
                "dog",
                "pig",
                "bear",
                ,
                ,
            ),
            "bear"
        );
    }

    #[test]
    fn test_max() {
        // usize
        assert_eq!(
            max!(1_usize, 6, 4, 2, 7, 0),
            7,
        );

        // isize
        assert_eq!(
            max!(6_isize, -7, -200, 4, 9000),
            9000,
        );

        // f64
        assert_eq!(
            max!(
                8.91_f64,
                92.0_f64,
                55.2_f64,
                1e100_f64,
            ),
            1e100
        );

        // &str
        assert_eq!(
            max!(
                "cat",
                "dog",
                "pig",
                "bear",
                ,
                ,
            ),
            "pig"
        );
    }
}
