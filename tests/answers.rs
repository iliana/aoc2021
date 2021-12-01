macro_rules! test {
    ($day:expr, $part1:expr, $part2:expr) => {
        paste::paste! {
            #[test]
            fn [<test_day $day>]() {
                assert_cmd::Command::cargo_bin(concat!("day", $day))
                    .unwrap()
                    .assert()
                    .success()
                    .stdout(concat!("part 1: ", $part1, "\npart 2: ", $part2, "\n"));
            }
        }
    };
}

test!(1, 1715, 1739);
