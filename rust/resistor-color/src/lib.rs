use enum_iterator::{all, Sequence};
use int_enum::IntEnum;

#[repr(usize)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, IntEnum, Sequence)]
pub enum ResistorColor {
    Black = 0,
    Brown = 1,
    Red = 2,
    Orange = 3,
    Yellow = 4,
    Green = 5,
    Blue = 6,
    Violet = 7,
    Grey = 8,
    White = 9,
}

pub fn color_to_value(_color: ResistorColor) -> usize {
    _color.int_value()
}

pub fn value_to_color_string(value: usize) -> String {
    match ResistorColor::from_int(value) {
        Ok(smth) => format!("{:?}", smth),
        _ => String::from("value out of range"),
    }
}

pub fn colors() -> Vec<ResistorColor> {
    all::<ResistorColor>().collect()
}
