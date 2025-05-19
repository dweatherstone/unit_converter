use unitconvert::convert::UnitConverter;
use unitconvert::error::ConvertError;

/// Compare two floats with tolerance
pub fn assert_approx_eq(a: f64, b: f64, tol: f64) {
    assert!(
        (a - b).abs() < tol,
        "Expected {} ≈ {}, difference {} > {}",
        a,
        b,
        (a - b).abs(),
        tol
    );
}

pub fn assert_convert_error<T, F>(converter: T, from: &str, to: &str, value: f64, matcher: F)
where
    T: UnitConverter,
    F: Fn(&ConvertError) -> bool,
{
    let result = converter.convert(value, from, to);
    assert!(result.is_err(), "Expected error, got {:?}", result);
    let err = result.unwrap_err();
    assert!(matcher(&err), "Unexpected error type: {:?}", err);
}
