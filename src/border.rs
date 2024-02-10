use libccanvas::bindings::Colour;
use serde::Deserialize;

#[derive(Deserialize)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct Border {
    pub colour: Colour,
    #[serde(flatten)]
    pub r#type: BorderType,
}

#[derive(Deserialize)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[serde(tag = "type")]
pub enum BorderType {
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "rounded")]
    Rounded,
    #[serde(rename = "double")]
    Double,
    #[serde(rename = "thick")]
    Thick,
    #[serde(rename = "custom")]
    Custom {
        left: char,
        topleft: char,
        top: char,
        topright: char,
        right: char,
        bottomright: char,
        bottom: char,
        bottomleft: char,
    },
}

impl BorderType {
    pub fn left(&self) -> char {
        match self {
            Self::Normal | Self::Rounded => '│',
            Self::Thick => '┃',
            Self::Double => '║',
            Self::Custom { left, .. } => *left,
        }
    }

    pub fn topleft(&self) -> char {
        match self {
            Self::Normal => '┌',
            Self::Rounded => '╭',
            Self::Thick => '┏',
            Self::Double => '╔',
            Self::Custom { topleft, .. } => *topleft,
        }
    }

    pub fn top(&self) -> char {
        match self {
            Self::Normal | Self::Rounded => '─',
            Self::Thick => '━',
            Self::Double => '═',
            Self::Custom { top, .. } => *top,
        }
    }

    pub fn topright(&self) -> char {
        match self {
            Self::Normal => '┐',
            Self::Rounded => '╮',
            Self::Thick => '┓',
            Self::Double => '╗',
            Self::Custom { topright, .. } => *topright,
        }
    }

    pub fn right(&self) -> char {
        match self {
            Self::Normal | Self::Rounded => '│',
            Self::Thick => '┃',
            Self::Double => '║',
            Self::Custom { right, .. } => *right,
        }
    }

    pub fn bottomright(&self) -> char {
        match self {
            Self::Normal => '┘',
            Self::Rounded => '╯',
            Self::Thick => '┛',
            Self::Double => '╝',
            Self::Custom { bottomright, .. } => *bottomright,
        }
    }

    pub fn bottom(&self) -> char {
        match self {
            Self::Normal | Self::Rounded => '─',
            Self::Thick => '━',
            Self::Double => '═',
            Self::Custom { bottom, .. } => *bottom,
        }
    }

    pub fn bottomleft(&self) -> char {
        match self {
            Self::Normal => '└',
            Self::Rounded => '╰',
            Self::Thick => '┗',
            Self::Double => '╚',
            Self::Custom { bottomleft, .. } => *bottomleft,
        }
    }
}

pub struct BorderSet {
    pub left: char,
    pub topleft: char,
    pub top: char,
    pub topright: char,
    pub right: char,
    pub bottomright: char,
    pub bottom: char,
    pub bottomleft: char,
}

impl From<&BorderType> for BorderSet {
    fn from(value: &BorderType) -> Self {
        Self {
            left: value.left(),
            topleft: value.topleft(),
            top: value.top(),
            topright: value.topright(),
            right: value.right(),
            bottomright: value.bottomright(),
            bottom: value.bottom(),
            bottomleft: value.bottomleft(),
        }
    }
}
