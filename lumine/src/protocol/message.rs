use std::fmt;

use serde::{Deserialize, Serialize};

macro_rules! make_cqcode_pattern {
    (not-first,$final:expr)=>{
        concat!(
            ",",
            stringify!($final),
            "={}]"
        )
    };
    (not-first,$left:expr,$($right:expr),+)=>{
        concat!(
            ",",
            stringify!($left),
            "={}",
            make_cqcode_pattern!(not-first,$($right),+)
        )
    };
    ($type:expr,$($other:expr),+) => {
        concat!(
            "[CQ:",
            stringify!($type),
            make_cqcode_pattern!(not-first,$($other),+)
        )
    };

}

/// format_cqcode!(formatter, cqcode_type, param1, param2, ...)
/// ```
/// use std::fmt::{Error, Write};
///
/// let mut buf = String::new();
/// let (id, name) = ("123", "qaq");
/// format_cqcode!(buf, poke, id, name)
/// assert_eq!(&buf, "[CQ:poke,id=123,name=qaq]");
/// ```
macro_rules! format_cqcode {
    ($fmt:expr,$type:expr,$($other:expr),+) => {
        write!($fmt,make_cqcode_pattern!($type,$($other),+),$($other),+)
    };
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum MessageSegment {
    Text {
        text: String,
    },
    Image {
        file: String,
    },
    Face {
        id: String,
    },
    Poke {
        id: String,
        name: String,
    },
    Share {
        url: String,
        title: String,
        content: String,
        image: String,
    },
}

impl fmt::Display for MessageSegment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MessageSegment::Text { text } => write!(f, "{}", text),
            MessageSegment::Image { file } => format_cqcode!(f, image, file),
            MessageSegment::Face { id } => format_cqcode!(f, face, id),
            MessageSegment::Poke { id, name } => format_cqcode!(f, poke, id, name),
            MessageSegment::Share {
                url,
                title,
                content,
                image,
            } => format_cqcode!(f, share, url, title, content, image),
            // _ => write!(f, "[unknown segment]"),
        }
    }
}
