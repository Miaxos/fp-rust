/// Pipe macro
/// Rewrite it to cleaner code with recursion
macro_rules! pipe {
    ($a:expr) => {
        $a
    };

    ($a:expr, $ab:expr) => {{
        $ab($a)
    }};

    ($a:expr, $ab:expr, $($bc:expr),+) => {{
        pipe!($ab($a), $($bc),*)
    }};
}
