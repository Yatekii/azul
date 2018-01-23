//! Contains utilities to convert strings (CSS strings) to servo types

use webrender::api::{ColorU, BorderRadius, LayoutSize};
use std::num::{ParseIntError, ParseFloatError};

pub const EM_HEIGHT: f32 = 16.0;

#[derive(Debug, PartialEq, Eq)]
pub enum CssBorderRadiusParseError<'a> {
    TooManyValues(&'a str),
    InvalidComponent(&'a str),
    ValueParseErr(ParseFloatError),
}

pub fn parse_border_radius<'a>(input: &'a str)
-> Result<BorderRadius, CssBorderRadiusParseError<'a>>
{
    let mut components = input.split_whitespace();
    let len = components.clone().count();

    match len {
        1 => {
            // One value - border-radius: 15px;
            // (the value applies to all four corners, which are rounded equally:

            let uniform_radius = parse_single_css_value(components.next().unwrap())?.to_pixels();
            Ok(BorderRadius::uniform(uniform_radius))
        },
        2 => {
            // Two values - border-radius: 15px 50px;
            // (first value applies to top-left and bottom-right corners,
            // and the second value applies to top-right and bottom-left corners):

            let top_left_bottom_right = parse_single_css_value(components.next().unwrap())?.to_pixels();
            let top_right_bottom_left = parse_single_css_value(components.next().unwrap())?.to_pixels();

            Ok(BorderRadius{
                top_left: LayoutSize::new(top_left_bottom_right, top_left_bottom_right),
                bottom_right: LayoutSize::new(top_left_bottom_right, top_left_bottom_right),
                top_right: LayoutSize::new(top_right_bottom_left, top_right_bottom_left),
                bottom_left: LayoutSize::new(top_right_bottom_left, top_right_bottom_left),
            })
        },
        3 => {
            // Three values - border-radius: 15px 50px 30px;
            // (first value applies to top-left corner,
            // second value applies to top-right and bottom-left corners,
            // and third value applies to bottom-right corner):
            let top_left = parse_single_css_value(components.next().unwrap())?.to_pixels();
            let top_right_bottom_left = parse_single_css_value(components.next().unwrap())?.to_pixels();
            let bottom_right = parse_single_css_value(components.next().unwrap())?.to_pixels();

            Ok(BorderRadius{
                top_left: LayoutSize::new(top_left, top_left),
                bottom_right: LayoutSize::new(bottom_right, bottom_right),
                top_right: LayoutSize::new(top_right_bottom_left, top_right_bottom_left),
                bottom_left: LayoutSize::new(top_right_bottom_left, top_right_bottom_left),
            })
        }
        4 => {
            // Four values - border-radius: 15px 50px 30px 5px;
            // (first value applies to top-left corner,
            //  second value applies to top-right corner,
            //  third value applies to bottom-right corner,
            //  fourth value applies to bottom-left corner)
            let top_left = parse_single_css_value(components.next().unwrap())?.to_pixels();
            let top_right = parse_single_css_value(components.next().unwrap())?.to_pixels();
            let bottom_right = parse_single_css_value(components.next().unwrap())?.to_pixels();
            let bottom_left = parse_single_css_value(components.next().unwrap())?.to_pixels();

            Ok(BorderRadius{
                top_left: LayoutSize::new(top_left, top_left),
                bottom_right: LayoutSize::new(bottom_right, bottom_right),
                top_right: LayoutSize::new(top_right, top_right),
                bottom_left: LayoutSize::new(bottom_left, bottom_left),
            })
        },
        _ => {
            Err(CssBorderRadiusParseError::TooManyValues(input))
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct PixelValue {
    metric: CssMetric,
    number: f32,
}

#[derive(Debug, PartialEq)]
pub enum CssMetric {
    Px,
    Em,
}

impl PixelValue {
    pub fn to_pixels(&self) -> f32 {
        match self.metric {
            CssMetric::Px => { self.number },
            CssMetric::Em => { self.number * EM_HEIGHT },
        }
    }
}

/// parse a single value such as "15px"
fn parse_single_css_value<'a>(input: &'a str)
-> Result<PixelValue, CssBorderRadiusParseError<'a>>
{
    let mut split_pos = 0;
    for (idx, ch) in input.char_indices() {
        if ch.is_numeric() || ch == '.' {
            split_pos = idx;
        }
    }

    split_pos += 1;

    let unit = &input[split_pos..];
    let unit = match unit {
        "px" => CssMetric::Px,
        "em" => CssMetric::Em,
        _ => { return Err(CssBorderRadiusParseError::InvalidComponent(&input[(split_pos - 1)..])); }
    };

    let number = input[..split_pos].parse::<f32>().map_err(|e| CssBorderRadiusParseError::ValueParseErr(e))?;

    Ok(PixelValue {
        metric: unit,
        number: number,
    })
}

#[derive(Debug, PartialEq)]
pub enum CssColorParseError<'a> {
    InvalidColor(&'a str),
    InvalidColorComponent(u8),
    ValueParseErr(ParseIntError),
}

pub fn parse_css_background_color<'a>(input: &'a str)
-> Result<ColorU, CssColorParseError<'a>>
{
    if input.starts_with('#') {
        parse_background_color(&input[1..])
    } else {
        parse_background_color_builtin(input)
    }
}

fn parse_background_color_builtin<'a>(input: &'a str)
-> Result<ColorU, CssColorParseError<'a>>
{
    let color = match input {
        "AliceBlue"              | "alice-blue"                 =>  "F0F8FF",
        "AntiqueWhite"           | "antique-white"              =>  "FAEBD7",
        "Aqua"                   | "aqua"                       =>  "00FFFF",
        "Aquamarine"             | "aquamarine"                 =>  "7FFFD4",
        "Azure"                  | "azure"                      =>  "F0FFFF",
        "Beige"                  | "beige"                      =>  "F5F5DC",
        "Bisque"                 | "bisque"                     =>  "FFE4C4",
        "Black"                  | "black"                      =>  "000000",
        "BlanchedAlmond"         | "blanched-almond"            =>  "FFEBCD",
        "Blue"                   | "blue"                       =>  "0000FF",
        "BlueViolet"             | "blue-violet"                =>  "8A2BE2",
        "Brown"                  | "brown"                      =>  "A52A2A",
        "BurlyWood"              | "burly-wood"                 =>  "DEB887",
        "CadetBlue"              | "cadet-blue"                 =>  "5F9EA0",
        "Chartreuse"             | "chartreuse"                 =>  "7FFF00",
        "Chocolate"              | "chocolate"                  =>  "D2691E",
        "Coral"                  | "coral"                      =>  "FF7F50",
        "CornflowerBlue"         | "cornflower-blue"            =>  "6495ED",
        "Cornsilk"               | "cornsilk"                   =>  "FFF8DC",
        "Crimson"                | "crimson"                    =>  "DC143C",
        "Cyan"                   | "cyan"                       =>  "00FFFF",
        "DarkBlue"               | "dark-blue"                  =>  "00008B",
        "DarkCyan"               | "dark-cyan"                  =>  "008B8B",
        "DarkGoldenRod"          | "dark-golden-rod"            =>  "B8860B",
        "DarkGray"               | "dark-gray"                  =>  "A9A9A9",
        "DarkGrey"               | "dark-grey"                  =>  "A9A9A9",
        "DarkGreen"              | "dark-green"                 =>  "006400",
        "DarkKhaki"              | "dark-khaki"                 =>  "BDB76B",
        "DarkMagenta"            | "dark-magenta"               =>  "8B008B",
        "DarkOliveGreen"         | "dark-olive-green"           =>  "556B2F",
        "DarkOrange"             | "dark-orange"                =>  "FF8C00",
        "DarkOrchid"             | "dark-orchid"                =>  "9932CC",
        "DarkRed"                | "dark-red"                   =>  "8B0000",
        "DarkSalmon"             | "dark-salmon"                =>  "E9967A",
        "DarkSeaGreen"           | "dark-sea-green"             =>  "8FBC8F",
        "DarkSlateBlue"          | "dark-slate-blue"            =>  "483D8B",
        "DarkSlateGray"          | "dark-slate-gray"            =>  "2F4F4F",
        "DarkSlateGrey"          | "dark-slate-grey"            =>  "2F4F4F",
        "DarkTurquoise"          | "dark-turquoise"             =>  "00CED1",
        "DarkViolet"             | "dark-violet"                =>  "9400D3",
        "DeepPink"               | "deep-pink"                  =>  "FF1493",
        "DeepSkyBlue"            | "deep-sky-blue"              =>  "00BFFF",
        "DimGray"                | "dim-gray"                   =>  "696969",
        "DimGrey"                | "dim-grey"                   =>  "696969",
        "DodgerBlue"             | "dodger-blue"                =>  "1E90FF",
        "FireBrick"              | "fire-brick"                 =>  "B22222",
        "FloralWhite"            | "floral-white"               =>  "FFFAF0",
        "ForestGreen"            | "forest-green"               =>  "228B22",
        "Fuchsia"                | "fuchsia"                    =>  "FF00FF",
        "Gainsboro"              | "gainsboro"                  =>  "DCDCDC",
        "GhostWhite"             | "ghost-white"                =>  "F8F8FF",
        "Gold"                   | "gold"                       =>  "FFD700",
        "GoldenRod"              | "golden-rod"                 =>  "DAA520",
        "Gray"                   | "gray"                       =>  "808080",
        "Grey"                   | "grey"                       =>  "808080",
        "Green"                  | "green"                      =>  "008000",
        "GreenYellow"            | "green-yellow"               =>  "ADFF2F",
        "HoneyDew"               | "honey-dew"                  =>  "F0FFF0",
        "HotPink"                | "hot-pink"                   =>  "FF69B4",
        "IndianRed"              | "indian-red"                 =>  "CD5C5C",
        "Indigo"                 | "indigo"                     =>  "4B0082",
        "Ivory"                  | "ivory"                      =>  "FFFFF0",
        "Khaki"                  | "khaki"                      =>  "F0E68C",
        "Lavender"               | "lavender"                   =>  "E6E6FA",
        "LavenderBlush"          | "lavender-blush"             =>  "FFF0F5",
        "LawnGreen"              | "lawn-green"                 =>  "7CFC00",
        "LemonChiffon"           | "lemon-chiffon"              =>  "FFFACD",
        "LightBlue"              | "light-blue"                 =>  "ADD8E6",
        "LightCoral"             | "light-coral"                =>  "F08080",
        "LightCyan"              | "light-cyan"                 =>  "E0FFFF",
        "LightGoldenRodYellow"   | "light-golden-rod-yellow"    =>  "FAFAD2",
        "LightGray"              | "light-gray"                 =>  "D3D3D3",
        "LightGrey"              | "light-grey"                 =>  "D3D3D3",
        "LightGreen"             | "light-green"                =>  "90EE90",
        "LightPink"              | "light-pink"                 =>  "FFB6C1",
        "LightSalmon"            | "light-salmon"               =>  "FFA07A",
        "LightSeaGreen"          | "light-sea-green"            =>  "20B2AA",
        "LightSkyBlue"           | "light-sky-blue"             =>  "87CEFA",
        "LightSlateGray"         | "light-slate-gray"           =>  "778899",
        "LightSlateGrey"         | "light-slate-grey"           =>  "778899",
        "LightSteelBlue"         | "light-steel-blue"           =>  "B0C4DE",
        "LightYellow"            | "light-yellow"               =>  "FFFFE0",
        "Lime"                   | "lime"                       =>  "00FF00",
        "LimeGreen"              | "lime-green"                 =>  "32CD32",
        "Linen"                  | "linen"                      =>  "FAF0E6",
        "Magenta"                | "magenta"                    =>  "FF00FF",
        "Maroon"                 | "maroon"                     =>  "800000",
        "MediumAquaMarine"       | "medium-aqua-marine"         =>  "66CDAA",
        "MediumBlue"             | "medium-blue"                =>  "0000CD",
        "MediumOrchid"           | "medium-orchid"              =>  "BA55D3",
        "MediumPurple"           | "medium-purple"              =>  "9370DB",
        "MediumSeaGreen"         | "medium-sea-green"           =>  "3CB371",
        "MediumSlateBlue"        | "medium-slate-blue"          =>  "7B68EE",
        "MediumSpringGreen"      | "medium-spring-green"        =>  "00FA9A",
        "MediumTurquoise"        | "medium-turquoise"           =>  "48D1CC",
        "MediumVioletRed"        | "medium-violet-red"          =>  "C71585",
        "MidnightBlue"           | "midnight-blue"              =>  "191970",
        "MintCream"              | "mint-cream"                 =>  "F5FFFA",
        "MistyRose"              | "misty-rose"                 =>  "FFE4E1",
        "Moccasin"               | "moccasin"                   =>  "FFE4B5",
        "NavajoWhite"            | "navajo-white"               =>  "FFDEAD",
        "Navy"                   | "navy"                       =>  "000080",
        "OldLace"                | "old-lace"                   =>  "FDF5E6",
        "Olive"                  | "olive"                      =>  "808000",
        "OliveDrab"              | "olive-drab"                 =>  "6B8E23",
        "Orange"                 | "orange"                     =>  "FFA500",
        "OrangeRed"              | "orange-red"                 =>  "FF4500",
        "Orchid"                 | "orchid"                     =>  "DA70D6",
        "PaleGoldenRod"          | "pale-golden-rod"            =>  "EEE8AA",
        "PaleGreen"              | "pale-green"                 =>  "98FB98",
        "PaleTurquoise"          | "pale-turquoise"             =>  "AFEEEE",
        "PaleVioletRed"          | "pale-violet-red"            =>  "DB7093",
        "PapayaWhip"             | "papaya-whip"                =>  "FFEFD5",
        "PeachPuff"              | "peach-puff"                 =>  "FFDAB9",
        "Peru"                   | "peru"                       =>  "CD853F",
        "Pink"                   | "pink"                       =>  "FFC0CB",
        "Plum"                   | "plum"                       =>  "DDA0DD",
        "PowderBlue"             | "powder-blue"                =>  "B0E0E6",
        "Purple"                 | "purple"                     =>  "800080",
        "RebeccaPurple"          | "rebecca-purple"             =>  "663399",
        "Red"                    | "red"                        =>  "FF0000",
        "RosyBrown"              | "rosy-brown"                 =>  "BC8F8F",
        "RoyalBlue"              | "royal-blue"                 =>  "4169E1",
        "SaddleBrown"            | "saddle-brown"               =>  "8B4513",
        "Salmon"                 | "salmon"                     =>  "FA8072",
        "SandyBrown"             | "sandy-brown"                =>  "F4A460",
        "SeaGreen"               | "sea-green"                  =>  "2E8B57",
        "SeaShell"               | "sea-shell"                  =>  "FFF5EE",
        "Sienna"                 | "sienna"                     =>  "A0522D",
        "Silver"                 | "silver"                     =>  "C0C0C0",
        "SkyBlue"                | "sky-blue"                   =>  "87CEEB",
        "SlateBlue"              | "slate-blue"                 =>  "6A5ACD",
        "SlateGray"              | "slate-gray"                 =>  "708090",
        "SlateGrey"              | "slate-grey"                 =>  "708090",
        "Snow"                   | "snow"                       =>  "FFFAFA",
        "SpringGreen"            | "spring-green"               =>  "00FF7F",
        "SteelBlue"              | "steel-blue"                 =>  "4682B4",
        "Tan"                    | "tan"                        =>  "D2B48C",
        "Teal"                   | "teal"                       =>  "008080",
        "Thistle"                | "thistle"                    =>  "D8BFD8",
        "Tomato"                 | "tomato"                     =>  "FF6347",
        "Turquoise"              | "turquoise"                  =>  "40E0D0",
        "Violet"                 | "violet"                     =>  "EE82EE",
        "Wheat"                  | "wheat"                      =>  "F5DEB3",
        "White"                  | "white"                      =>  "FFFFFF",
        "WhiteSmoke"             | "white-smoke"                =>  "F5F5F5",
        "Yellow"                 | "yellow"                     =>  "FFFF00",
        "YellowGreen"            | "yellow-green"               =>  "9ACD32",
        _ => { return Err(CssColorParseError::InvalidColor(input)); }
    };
    parse_background_color(color)
}

/// parse a background color (assumes "00FFFF" -> ColorF { r: 0, g: 255, b: 255})
///
fn parse_background_color<'a>(input: &'a str)
-> Result<ColorU, CssColorParseError<'a>>
{
    #[inline]
    fn from_hex<'a>(c: u8) -> Result<u8, CssColorParseError<'a>> {
        match c {
            b'0' ... b'9' => Ok(c - b'0'),
            b'a' ... b'f' => Ok(c - b'a' + 10),
            b'A' ... b'F' => Ok(c - b'A' + 10),
            _ => Err(CssColorParseError::InvalidColorComponent(c))
        }
    }

    match input.len() {
        3 => {
            let mut input_iter = input.chars();

            let r = input_iter.next().unwrap() as u8;
            let g = input_iter.next().unwrap() as u8;
            let b = input_iter.next().unwrap() as u8;

            let r = from_hex(r)? * 16 + from_hex(r)?;
            let g = from_hex(g)? * 16 + from_hex(g)?;
            let b = from_hex(b)? * 16 + from_hex(b)?;

            Ok(ColorU {
                r: r,
                g: g,
                b: b,
                a: 255,
            })
        },
        4 => {
            let mut input_iter = input.chars();

            let r = input_iter.next().unwrap() as u8;
            let g = input_iter.next().unwrap() as u8;
            let b = input_iter.next().unwrap() as u8;
            let a = input_iter.next().unwrap() as u8;

            let r = from_hex(r)? * 16 + from_hex(r)?;
            let g = from_hex(g)? * 16 + from_hex(g)?;
            let b = from_hex(b)? * 16 + from_hex(b)?;
            let a = from_hex(a)? * 16 + from_hex(a)?;

            Ok(ColorU {
                r: r,
                g: g,
                b: b,
                a: a,
            })
        },
        6 => {
            let input = u32::from_str_radix(input, 16).map_err(|e| CssColorParseError::ValueParseErr(e))?;
            Ok(ColorU {
                r: ((input >> 16) & 255) as u8,
                g: ((input >> 8) & 255) as u8,
                b: (input & 255) as u8,
                a: 255,
            })
        },
        8 => {
            let input = u32::from_str_radix(input, 16).map_err(|e| CssColorParseError::ValueParseErr(e))?;
            Ok(ColorU {
                r: ((input >> 24) & 255) as u8,
                g: ((input >> 16) & 255) as u8,
                b: ((input >> 8) & 255) as u8,
                a: (input & 255) as u8,
            })
        },
        _ => { Err(CssColorParseError::InvalidColor(input)) }
    }
}

#[test]
fn test_parse_background_color() {
    assert_eq!(parse_css_background_color("#F0F8FF"), Ok(ColorU { r: 240, g: 248, b: 255, a: 255 }));
    assert_eq!(parse_css_background_color("#F0F8FF00"), Ok(ColorU { r: 240, g: 248, b: 255, a: 0 }));
    assert_eq!(parse_css_background_color("#EEE"), Ok(ColorU { r: 238, g: 238, b: 238, a: 255 }));
}


#[test]
fn test_parse_single_css_value() {
    assert_eq!(parse_single_css_value("15px"), Ok(PixelValue { metric: CssMetric::Px, number: 15.0 }));
    assert_eq!(parse_single_css_value("1.2em"), Ok(PixelValue { metric: CssMetric::Em, number: 1.2 }));
    assert_eq!(parse_single_css_value("aslkfdjasdflk"), Err(CssBorderRadiusParseError::InvalidComponent("aslkfdjasdflk")));
}

#[test]
fn test_parse_border_radius() {
    assert_eq!(parse_border_radius("15px"), Ok(BorderRadius::uniform(15.0)));
    assert_eq!(parse_border_radius("15px 50px"), Ok(BorderRadius {
        top_left: LayoutSize::new(15.0, 15.0),
        bottom_right: LayoutSize::new(15.0, 15.0),
        top_right: LayoutSize::new(50.0, 50.0),
        bottom_left: LayoutSize::new(50.0, 50.0),
    }));
    assert_eq!(parse_border_radius("15px 50px 30px"), Ok(BorderRadius {
        top_left: LayoutSize::new(15.0, 15.0),
        bottom_right: LayoutSize::new(30.0, 30.0),
        top_right: LayoutSize::new(50.0, 50.0),
        bottom_left: LayoutSize::new(50.0, 50.0),
    }));
    assert_eq!(parse_border_radius("15px 50px 30px 5px"), Ok(BorderRadius {
        top_left: LayoutSize::new(15.0, 15.0),
        bottom_right: LayoutSize::new(30.0, 30.0),
        top_right: LayoutSize::new(50.0, 50.0),
        bottom_left: LayoutSize::new(5.0, 5.0),
    }));
}