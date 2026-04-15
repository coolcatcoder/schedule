#![feature(generic_const_items)]
#![feature(min_generic_const_args)]
#![feature(generic_const_args)]
#![expect(incomplete_features)]

use core::range::Range;
use regex::Regex;
use std::{fmt::Write, ops::Shr};

mod horrors;

fn what() {
    let bad = || || || || || || || .. .. .. .. .. .. .. .. .. .. .. .. ..;
}

trait NextInString {
    fn next<'a>(&self, string: &'a str) -> Option<SubStr<'a>>;
}

enum Repeat {
    Unbound,
    Bound(u32),
}

enum MatchSegmentInternal {
    Unused,
    Regex(Regex),
    RepeatLazy {
        regex: Regex,
        minimum: Repeat,
        maximum: Repeat,
    },
}

impl From<&str> for MatchSegmentInternal {
    fn from(value: &str) -> Self {
        Self::Regex(Regex::new(value).unwrap())
    }
}
impl From<std::ops::RangeFrom<&str>> for MatchSegmentInternal {
    fn from(value: std::ops::RangeFrom<&str>) -> Self {
        Self::RepeatLazy {
            regex: Regex::new(value.start).unwrap(),
            minimum: Repeat::Unbound,
            maximum: Repeat::Unbound,
        }
    }
}

#[derive(Clone, Copy)]
struct SubStr<'a> {
    from: &'a str,
    range: Range<usize>,
}

impl SubStr<'_> {
    fn as_str(&self) -> &str {
        &self.from[self.range]
    }
}

struct NewMatcher<const LENGTH: usize> {
    segments: [MatchSegmentInternal; LENGTH],
}
const M: NewMatcher<0> = NewMatcher { segments: [] };

impl<const LENGTH: usize> NewMatcher<LENGTH> {
    fn to_regex(&self) -> Regex {
        let mut regex_builder = String::new();
        for segment in &self.segments {
            match segment {
                MatchSegmentInternal::Regex(regex) => {
                    regex_builder += regex.as_str();
                }
                MatchSegmentInternal::RepeatLazy {
                    regex,
                    minimum,
                    maximum,
                } => {
                    // TODO: For now we are ignoring min and max.
                    write!(regex_builder, "(?:(?:{})*?)", regex.as_str()).unwrap();
                }
                MatchSegmentInternal::Unused => panic!(),
            }
        }
        Regex::new(&regex_builder).unwrap()
    }
}

macro_rules! type_const {
    ($($any:tt)*) => {
        type const $($any)*;
    };
}

type_const!(ADD<const A: usize, const B: usize>: usize = const { A + B });

impl<const LENGTH: usize, T: Into<MatchSegmentInternal>> Shr<T> for NewMatcher<LENGTH> {
    type Output = NewMatcher<{ ADD::<LENGTH, 1> }>;

    fn shr(self, rhs: T) -> Self::Output {
        let mut array = [const { MatchSegmentInternal::Unused }; { ADD::<LENGTH, 1> }];
        for (index, value) in self.segments.into_iter().enumerate() {
            array[index] = value;
        }
        array[LENGTH] = rhs.into();
        NewMatcher { segments: array }
    }
}

struct Parser<'a> {
    string: &'a str,
    cursor: usize,
}
#[allow(nonstandard_style)]
const Parser: fn(&str) -> Parser = |string| Parser { string, cursor: 0 };

impl<'a> Parser<'a> {
    fn next<const LENGTH: usize>(
        &mut self,
        matcher: &NewMatcher<LENGTH>,
    ) -> Option<[SubStr<'a>; LENGTH]> {
        fn match_segment<'a>(
            parser: &Parser<'a>,
            index: usize,
            segments: &[MatchSegmentInternal],
        ) -> Option<SubStr<'a>> {
            match &segments[index] {
                MatchSegmentInternal::Unused => panic!("Unused should never appear."),
                MatchSegmentInternal::Regex(regex) => {
                    let matched = regex.find_at(parser.string, parser.cursor)?;
                    Some(SubStr {
                        from: parser.string,
                        range: matched.range().into(),
                    })
                }
                MatchSegmentInternal::RepeatLazy {
                    regex,
                    minimum,
                    maximum,
                } => {
                    let peeked = match_segment(parser, index + 1, segments)?;
                    let cursor_to_start_of_peeked = parser.cursor..peeked.range.start;
                    let cursor_to_start_of_peeked_length = cursor_to_start_of_peeked.len();

                    if let Repeat::Bound(minimum) = minimum
                        && cursor_to_start_of_peeked_length < *minimum as usize
                    {
                        return None;
                    };
                    if let Repeat::Bound(maximum) = maximum
                        && cursor_to_start_of_peeked_length > *maximum as usize
                    {
                        return None;
                    };

                    for character in parser.string[cursor_to_start_of_peeked.clone()].chars() {
                        let mut character_to_ampersand_str_buffer = [0; 4];
                        let character =
                            character.encode_utf8(&mut character_to_ampersand_str_buffer);

                        if !regex.is_match(character) {
                            return None;
                        }
                    }

                    Some(SubStr {
                        from: parser.string,
                        range: cursor_to_start_of_peeked.into(),
                    })
                }
            }
        }

        let mut matches = [SubStr {
            from: self.string,
            range: Range::default(),
        }; LENGTH];

        for (index, matches) in matches.iter_mut().enumerate() {
            let matched = match_segment(self, index, &matcher.segments)?;

            self.cursor = matched.range.end;
            matches.range = matched.range;
        }

        Some(matches)
    }
}

fn basic() {
    let file = include_str!("/home/coolcatcoder/Documents/GitHub/random_notes/My Path Forward.md");
    let mut parser = Parser {
        string: file,
        cursor: 0,
    };

    let properties_matcher = M >> "---" >> "---";
    let start_of_property = (M >> "\n" >> ("."..) >> ":").to_regex();
    let start_of_property = M >> MatchSegmentInternal::Regex(start_of_property);

    // let start_of_property = (M >> "\n" >> ("."..) >> ":").to_regex();
    // println!("Start of Property:\n{:?}", start_of_property.as_str());
    // let property = start_of_property.find(file).unwrap();
    // println!("First Property:\n{:?}", property.as_str());

    // TODO: Introduce a new operator, as we need blocks for this to work. Find the next place that this whole thing matches.
    //let property_matcher = M >> "\n" >> ("."..) >> ":" >> "\n" >> ("."..) >> ":";

    // let property_matcher =
    //     M >> "\n" >> ("."..) >> ":" >> MatchSegmentInternal::Regex(start_of_property);

    // TODO: IDEAL!!!
    // let start_of_property = M >> "\n" >> ("."..) >> ":";
    // let mut properties = vec![];
    // while let Some(property) = parser.next(&start_of_property) {
    //     properties.push(property);
    // }

    let [start, end] = parser.next(&properties_matcher).unwrap();
    let between = &file[start.range.end..end.range.start];
    println!("Between:\n{}", between);

    let mut between_parser = Parser(between);
    // let property = between_parser.next(&property_matcher).unwrap();
    // for sub_str in property {
    //     println!("Matched:\n{:?}", sub_str.as_str());
    // }

    let mut properties = vec![];
    while let Some(property) = between_parser.next(&start_of_property) {
        properties.push(property);
    }

    for property in properties {
        println!("Property:\n{:?}", property[0].as_str());
    }

    let weird = .. .. .. .. .. .. .. .. .. .. .. .. .. .. ..;
    let why: std::ops::RangeFrom<std::ops::RangeFull> = (..)..;
}

#[cfg(test)]
mod tests {
    #[test]
    fn basic() {
        super::basic();
    }
}
