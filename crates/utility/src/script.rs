// This file is part of `i18n_utility-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_utility-rizzen-yazston` crate.

#[derive(Debug, Clone, PartialEq)]
pub enum Direction {
    TopToBottom,
    BottomToTop,
    LeftToRight,
    RightToLeft,
}

impl Direction {
    #[allow(dead_code)]
    fn as_str(&self) -> &str {
        match self {
            Direction::TopToBottom => "TopToBottom",
            Direction::BottomToTop => "BottomToTop",
            Direction::LeftToRight => "LeftToRight",
            Direction::RightToLeft => "RightToLeft",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ScriptDirection {
    TopToBottomLeftToRight,
    TopToBottomRightToLeft,
    BottomToTopLeftToRight,
    BottomToTopRightToLeft,
    LeftToRightTopToBottom,
    LeftToRightBottomToTop,
    RightToLeftTopToBottom,
    RightToLeftBottomToTop,
}

impl ScriptDirection {
    pub fn directions(&self) -> (Direction, Direction) {
        match self {
            ScriptDirection::TopToBottomLeftToRight => {
                (Direction::TopToBottom, Direction::LeftToRight)
            }
            ScriptDirection::TopToBottomRightToLeft => {
                (Direction::TopToBottom, Direction::RightToLeft)
            }
            ScriptDirection::BottomToTopLeftToRight => {
                (Direction::BottomToTop, Direction::LeftToRight)
            }
            ScriptDirection::BottomToTopRightToLeft => {
                (Direction::BottomToTop, Direction::RightToLeft)
            }
            ScriptDirection::LeftToRightTopToBottom => {
                (Direction::LeftToRight, Direction::TopToBottom)
            }
            ScriptDirection::LeftToRightBottomToTop => {
                (Direction::LeftToRight, Direction::BottomToTop)
            }
            ScriptDirection::RightToLeftTopToBottom => {
                (Direction::RightToLeft, Direction::TopToBottom)
            }
            ScriptDirection::RightToLeftBottomToTop => {
                (Direction::RightToLeft, Direction::BottomToTop)
            }
        }
    }

    pub fn line(&self) -> Direction {
        match self {
            ScriptDirection::TopToBottomLeftToRight => Direction::TopToBottom,
            ScriptDirection::TopToBottomRightToLeft => Direction::TopToBottom,
            ScriptDirection::BottomToTopLeftToRight => Direction::BottomToTop,
            ScriptDirection::BottomToTopRightToLeft => Direction::BottomToTop,
            ScriptDirection::LeftToRightTopToBottom => Direction::LeftToRight,
            ScriptDirection::LeftToRightBottomToTop => Direction::LeftToRight,
            ScriptDirection::RightToLeftTopToBottom => Direction::RightToLeft,
            ScriptDirection::RightToLeftBottomToTop => Direction::RightToLeft,
        }
    }

    pub fn grapheme(&self) -> Direction {
        match self {
            ScriptDirection::TopToBottomLeftToRight => Direction::LeftToRight,
            ScriptDirection::TopToBottomRightToLeft => Direction::RightToLeft,
            ScriptDirection::BottomToTopLeftToRight => Direction::LeftToRight,
            ScriptDirection::BottomToTopRightToLeft => Direction::RightToLeft,
            ScriptDirection::LeftToRightTopToBottom => Direction::TopToBottom,
            ScriptDirection::LeftToRightBottomToTop => Direction::BottomToTop,
            ScriptDirection::RightToLeftTopToBottom => Direction::TopToBottom,
            ScriptDirection::RightToLeftBottomToTop => Direction::BottomToTop,
        }
    }

    #[allow(dead_code)]
    fn as_str(&self) -> &str {
        match self {
            ScriptDirection::TopToBottomLeftToRight => "TopToBottomLeftToRight",
            ScriptDirection::TopToBottomRightToLeft => "TopToBottomRightToLeft",
            ScriptDirection::BottomToTopLeftToRight => "BottomToTopLeftToRight",
            ScriptDirection::BottomToTopRightToLeft => "BottomToTopRightToLeft",
            ScriptDirection::LeftToRightTopToBottom => "LeftToRightTopToBottom",
            ScriptDirection::LeftToRightBottomToTop => "LeftToRightBottomToTop",
            ScriptDirection::RightToLeftTopToBottom => "RightToLeftTopToBottom",
            ScriptDirection::RightToLeftBottomToTop => "RightToLeftBottomToTop",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Category {
    Logographic, // Includes Hieroglyphic and Cuneiform scripts
    Alphabetic,
    Abugida, // Also known as Alphasyllabary
    Abjad,   // Includes Shorthand scripts
    Syllabic,
    Ideographic, // Includes Japanese and Chinese scripts
    Special,     // Includes private use codes
}

impl Category {
    #[allow(dead_code)]
    fn as_str(&self) -> &str {
        match self {
            Category::Logographic => "Logographic",
            Category::Alphabetic => "Alphabetic",
            Category::Abugida => "Abugida",
            Category::Abjad => "Abjad",
            Category::Syllabic => "Syllabic",
            Category::Ideographic => "Ideographic",
            Category::Special => "Special",
        }
    }
}

#[derive(Debug, Clone)]
pub struct ScriptData {
    pub category: Category,
    pub directions: Vec<ScriptDirection>,
    pub historic: bool,
}
