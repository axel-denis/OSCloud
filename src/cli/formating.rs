use colored::{ColoredString, Colorize};
use std::string::ToString;

pub(crate) fn err_str<T: ToString>(text: T) -> ColoredString {
    text.to_string().red().bold()
}

pub(crate) fn ok_str<T: ToString>(text: T) -> ColoredString {
    text.to_string().green().bold()
}

pub(crate) fn info_str<T: ToString>(text: T) -> ColoredString {
    text.to_string().bright_black()
}
