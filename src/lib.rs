pub use libc_print::*;

#[macro_export]
macro_rules! input {
    (@__prefix $prefix:expr) => {
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/input/",
            $prefix,
            env!("CARGO_BIN_NAME"),
            ".txt",
        ))
    };

    () => {{
        #[cfg(test)] { input!(@__prefix "test_") }
        #[cfg(not(test))] { input!(@__prefix "") }
    }};

    ($ty:ty) => {{
        use core::str::FromStr;
        input!().lines().map(|l| <$ty>::from_str(l).unwrap())
    }};

    ($ty:ty, $split:expr) => {
        input!().split($split).map(|l| <$ty>::from_str(l).unwrap())
    };
}
