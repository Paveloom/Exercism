use enum_iterator::IntoEnumIterator;
use int_enum::IntEnum;

#[repr(usize)]
#[derive(Copy, Clone, Debug, IntEnum, IntoEnumIterator, PartialEq)]
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

#[must_use]
pub fn color_to_value(color: ResistorColor) -> usize {
    color as usize
}

#[must_use]
pub fn value_to_color_string(value: usize) -> String {
    ResistorColor::from_int(value).map_or(String::from("value out of range"), |var| {
        format!("{:?}", var)
    })
}

#[must_use]
pub fn colors() -> Vec<ResistorColor> {
    ResistorColor::into_enum_iter().collect()
}
