use core::time::Duration;
use time::PrimitiveDateTime as DateTime;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    start + time::Duration::seconds((10 as i64).pow(9))
}
