use crate::Error::{InvalidDigit, InvalidInputBase, InvalidOutputBase};

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return an `Err(.)` if the conversion is impossible.
/// The tests do not test for specific values inside the `Err(.)`.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    // Validation
    if from_base < 2 {
        return Err(InvalidInputBase);
    }
    if to_base < 2 {
        return Err(InvalidOutputBase);
    }
    let errors = number
        .iter()
        .filter(|d| *d >= &from_base)
        .collect::<Vec<&u32>>();
    if !errors.is_empty() {
        return Err(InvalidDigit(**errors.get(0).unwrap()));
    }

    let mut resolved_number: u32 = number
        .iter()
        .rev()
        .enumerate()
        .map(|(i, d)| d * from_base.pow(i as u32))
        .sum();
    if resolved_number == 0 {
        return Ok(vec![0]);
    }

    let mut result: Vec<u32> = vec![];
    while resolved_number > 0 {
        result.push(resolved_number % to_base);
        resolved_number /= to_base;
    }
    result.reverse();
    Ok(result)
}
