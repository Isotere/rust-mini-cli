pub fn fahrenheit_to_celsius(value: f64) -> f64 {
    (value - 32.0) * (5.0 / 9.0)
}

pub fn celsius_to_fahrenheit(value: f64) -> f64 {
    (value * 9.0 / 5.0) + 32.0
}

#[cfg(test)]
mod tests {
    use assert_float_eq::assert_float_absolute_eq;

    // 0°C=32°F, 100°C=212°F, -40°C=-40°F
    use super::*;

    #[test]
    fn test_fah2cel() {
        let res = fahrenheit_to_celsius(32.0);
        assert_float_absolute_eq!(res, 0.0);

        let res = fahrenheit_to_celsius(212.0);
        assert_float_absolute_eq!(res, 100.0);

        let res = fahrenheit_to_celsius(-40.0);
        assert_float_absolute_eq!(res, -40.0);
    }

    #[test]
    fn test_cel2fah() {
        let res = celsius_to_fahrenheit(0.0);
        assert_float_absolute_eq!(res, 32.0);

        let res = celsius_to_fahrenheit(100.0);
        assert_float_absolute_eq!(res, 212.0);

        let res = celsius_to_fahrenheit(-40.0);
        assert_float_absolute_eq!(res, -40.0);
    }
}
