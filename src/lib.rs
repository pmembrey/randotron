#[macro_export]
macro_rules! randotron {
    ($sample_rate:expr, $closure:expr) => {{
        // Simple short circuiting logic
        match $sample_rate {
           // This would cause a divide by zero, but it's a handy way to prevent the code from running
            0 => None,
            // This would always hit, so skip the random generation and just run the closure
            1 => {
                // Execute the closure
                let _res = $closure();
                Some(_res)
            },
            // Otherwise, let's do the actual work
            rate => {
                // Using this library for simplicity
                use random_number::random;

                // Get a random 64 bit integer
                let temp: u64 = random!();

                // Did we get lucky?
                if temp % rate == 0 {
                    // Execute the closure
                    let _res = $closure();
                    Some(_res)
                } else {
                    None
                }
            }
        }
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_certain() {
        let result = randotron!(1, || { "foo" });
        assert_eq!(Some("foo"), result);
    }
    #[test]
    fn test_zero_sample_rate() {
        let result = randotron!(0, || { "bar" });
        assert_eq!(None, result);
    }
}
