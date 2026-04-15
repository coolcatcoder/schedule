#![feature(generic_const_items)]
#![feature(min_generic_const_args)]
#![feature(generic_const_args)]
#![expect(incomplete_features)]
#![feature(array_try_map)]

use core::range::Range;
use regex::Regex;
use std::{fmt::Write, ops::Shr};

mod horrors;

struct RegexBuilder<T = Regex>(T);
const R: RegexBuilder<()> = RegexBuilder(());

impl<T: ToRegex> Shr<T> for RegexBuilder<()> {
    type Output = RegexBuilder;

    fn shr(self, rhs: T) -> Self::Output {
        let mut string = String::new();
        rhs.to_regex(&mut string);
        RegexBuilder(Regex::new(&string).unwrap())
    }
}
impl<T: ToRegex> Shr<T> for RegexBuilder {
    type Output = Self;

    fn shr(self, rhs: T) -> Self::Output {
        let mut string = self.0.as_str().to_owned();
        rhs.to_regex(&mut string);
        RegexBuilder(Regex::new(&string).unwrap())
    }
}

trait ToRegex {
    fn to_regex(self, string: &mut String) -> Result<(), std::fmt::Error>;
}
impl ToRegex for &str {
    fn to_regex(self, string: &mut String) -> Result<(), std::fmt::Error> {
        write!(string, "{self}")
    }
}
impl ToRegex for std::ops::RangeFrom<&str> {
    fn to_regex(self, string: &mut String) -> Result<(), std::fmt::Error> {
        write!(string, "(?:(?:{})*?)", self.start)
    }
}
impl<const LENGTH: usize> ToRegex for [&str; LENGTH] {
    fn to_regex(self, string: &mut String) -> Result<(), std::fmt::Error> {
        write!(string, "(?:")?;

        let mut iterator = self.into_iter();
        let last = iterator.next_back().unwrap();
        for regex in iterator {
            write!(string, "(?:{regex})|")?;
        }
        write!(string, "(?:{last})")?;

        write!(string, ")")
    }
}

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

trait Matcher {
    type Output<T>;

    fn for_each_regex<'a>(
        self,
        f: impl FnMut(&Regex) -> Option<SubStr<'a>>,
    ) -> Option<Self::Output<SubStr<'a>>>;
}
impl Matcher for &RegexBuilder {
    type Output<T> = T;

    fn for_each_regex<'a>(
        self,
        mut f: impl FnMut(&Regex) -> Option<SubStr<'a>>,
    ) -> Option<Self::Output<SubStr<'a>>> {
        f(&self.0)
    }
}
impl<const LENGTH: usize> Matcher for [&RegexBuilder; LENGTH] {
    type Output<T> = [T; LENGTH];

    fn for_each_regex<'a>(
        self,
        mut f: impl FnMut(&Regex) -> Option<SubStr<'a>>,
    ) -> Option<Self::Output<SubStr<'a>>> {
        self.try_map(|regex| f(&regex.0))
    }
}

impl<'a> Parser<'a> {
    fn next<T: Matcher>(&mut self, matcher: T) -> Option<T::Output<SubStr<'_>>> {
        matcher.for_each_regex(|regex| {
            regex.find_at(self.string, self.cursor).map(|matched| {
                self.cursor = matched.end();
                SubStr {
                    from: self.string,
                    range: matched.range().into(),
                }
            })
        })
    }

    fn next_old<const LENGTH: usize>(
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

const END_OF_FILE: &str = r"\z";

fn basic() {
    let file = include_str!("/home/coolcatcoder/Documents/GitHub/random_notes/My Path Forward.md");
    let mut parser = Parser {
        string: file,
        cursor: 0,
    };

    let properties_matcher = R >> "---";
    let property_matcher = R >> "\n" >> ("."..) >> ":";

    let heading_matcher = R >> "\n# " >> ("."..) >> ["\n", END_OF_FILE];
    println!("Heading Matcher: {:?}", heading_matcher.0.as_str());

    if let Some([start, end]) = parser.next([&properties_matcher; 2]) {
        let between = &file[start.range.end..end.range.start];
        println!("Between:\n{}", between);

        let mut parser = Parser(between);

        while let Some(property) = parser.next(&property_matcher) {
            println!("Property:\n{:?}", property.as_str());
        }
    }

    while let Some(heading) = parser.next(&heading_matcher) {
        println!("Heading:\n{:?}", heading.as_str());
    }

    // let properties_matcher = M >> "---" >> "---";
    // let start_of_property = (M >> "\n" >> ("."..) >> ":").to_regex();
    // let start_of_property = M >> MatchSegmentInternal::Regex(start_of_property);

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

    // let [start, end] = parser.next(&properties_matcher).unwrap();
    // let between = &file[start.range.end..end.range.start];
    // println!("Between:\n{}", between);

    // let mut between_parser = Parser(between);
    // let property = between_parser.next(&property_matcher).unwrap();
    // for sub_str in property {
    //     println!("Matched:\n{:?}", sub_str.as_str());
    // }

    // let mut properties = vec![];
    // while let Some(property) = between_parser.next(&start_of_property) {
    //     properties.push(property);
    // }

    // for property in properties {
    //     println!("Property:\n{:?}", property[0].as_str());
    // }

    // let weird = .. .. .. .. .. .. .. .. .. .. .. .. .. .. ..;
    // let why: std::ops::RangeFrom<std::ops::RangeFull> = (..)..;
}

#[cfg(test)]
mod tests {
    #[test]
    fn basic() {
        super::basic();
    }
}
