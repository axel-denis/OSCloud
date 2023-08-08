use std::string::ToString;
use colored::{Colorize, ColoredString};

pub(crate) fn err_str<T: ToString>(text: T) -> ColoredString {
    text.to_string().red().bold()
}

pub(crate) fn ok_str<T: ToString>(text: T) -> ColoredString {
    text.to_string().green().bold()
}
