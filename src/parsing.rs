use crate::DetachedStr;
use bevy::prelude::*;
use indoc::indoc;
use regex::{Captures, Match, Regex};
use std::ops::Add;

enum MatchSegment {
    EndOfFile,
    Regex(&'static str),
    Any,
}

enum MatchSegmentInternal {
    EndOfFile,
    Regex(Regex),
    Any,
    Or,
}

impl From<MatchSegment> for MatchSegmentInternal {
    fn from(value: MatchSegment) -> Self {
        match value {
            MatchSegment::EndOfFile => MatchSegmentInternal::EndOfFile,
            MatchSegment::Regex(regex) => MatchSegmentInternal::Regex(Regex::new(regex).unwrap()),
            MatchSegment::Any => MatchSegmentInternal::Any,
        }
    }
}

struct NewMatcher<const LENGTH: usize> {
    segments: [MatchSegmentInternal; LENGTH],
}

macro_rules! type_const {
    ($($any:tt)*) => {
        type const $($any)*;
    };
}

type_const!(ADD<const A: usize, const B: usize>: usize = const { A + B });

impl<const LENGTH: usize> Add<MatchSegment> for NewMatcher<LENGTH> {
    type Output = NewMatcher<{ ADD::<LENGTH, 1> }>;

    fn add(self, rhs: MatchSegment) -> Self::Output {
        let mut array = [const { MatchSegmentInternal::Any }; { ADD::<LENGTH, 1> }];
        for (index, value) in self.segments.into_iter().enumerate() {
            array[index] = value;
        }
        array[LENGTH] = rhs.into();
        NewMatcher { segments: array }
    }
}

impl Add for MatchSegment {
    type Output = NewMatcher<2>;

    fn add(self, rhs: Self) -> Self::Output {
        NewMatcher {
            segments: [self.into(), rhs.into()],
        }
    }
}

fn parse_file_new(file: &str) {
    let mut parser = Parser(file);
    let better_regex = BetterRegex::new();

    use MatchSegment::*;
    let weird = M == "---" > "---";

    //let properties_matcher = Matcher::new("---{properties: Any}---", 0);
    let properties_matcher = Matcher::new("---((?:(?s).)*?)---", 0);
    let property_matcher = Matcher::new(
        indoc! {r"
        (?x)
        \n
        (
            (?:
                .
            )*?
        ):

        (
            (?:(?s)
                .
            )*?
        )

        (?:
            (?:
                \n
                (?:
                    .
                )*?
                :
            )|\z
        )
    "},
        2,
    );

    //let section_matcher = Matcher::new("\n# {any(), title}\n", 0);
    //let section_matcher = Matcher::new("\n# ((?:(?s).)*?)\n", 1);
    // r"\n# {title: Heading}(\n{body: Any}(\n# |\z)|\z)"
    // let section_matcher =
    //     better_regex.match_next(r"\n# {title: Heading}(\n{body: Any}(\n# |\z)|\z)", "body");
    let section_matcher = better_regex.match_next(
        // Doesn't work, because the end of body includes \n# .
        r"\n# {title: Heading}{body: Regex<(\n(.|\n)*?(\n# |\z))|\z>}",
        "body",
    );

    let properties = parser
        .consume(&properties_matcher)
        .map(|captures| {
            let mut parser = Parser(&captures[1]);
            let mut others = vec![];

            while let Some(captures) = parser.consume(&property_matcher) {
                let property = InternalProperty {
                    title: DetachedStr(file.substr_range(&captures[1]).unwrap()),
                    body: DetachedStr(file.substr_range(&captures[2]).unwrap()),
                };
                others.push(property);
            }

            Properties {
                tags: vec![],
                others,
            }
        })
        .unwrap_or_default();

    while let Some(captures) = parser.consume(&section_matcher) {
        info!("Section: {:?}", &captures["title"]);
        info!("Body: {:?}", &captures[2]);
    }

    info!("Remaining:\n{:?}", parser.0);
}

struct BetterRegex(Matcher<usize>);

impl BetterRegex {
    fn new() -> Self {
        let regex = Regex::new(indoc! {r"
            (?x)
            \{
                (
                    (?:
                        [[:alpha:]]
                    )*?
                ):(?:\ )?
                (
                    (?:
                        [[:alpha:]]
                    )*?
                )
                (?:
                    <((?:.)*?)>
                )?
            \}
        "})
        .unwrap();
        Self(Matcher {
            regex,
            end_index: 0,
        })
    }

    fn match_at_start<I>(&self, better_regex: &str, end_index: I) -> Matcher<I> {
        let mut regex = String::with_capacity(better_regex.len());
        regex.push_str(r"\A");
        self.match_internal(regex, better_regex, end_index)
    }

    fn match_next<I>(&self, better_regex: &str, end_index: I) -> Matcher<I> {
        let mut regex = String::with_capacity(better_regex.len());
        self.match_internal(regex, better_regex, end_index)
    }

    fn match_internal<I>(&self, mut regex: String, better_regex: &str, end_index: I) -> Matcher<I> {
        let better_regex = better_regex.replace('(', "(?:");
        let mut better_regex = better_regex.as_str();

        while let Some(captures) = self.0.regex.captures(better_regex) {
            regex.push_str(&better_regex[..captures.get_match().start()]);
            better_regex = &better_regex[captures.get_match().end()..];

            let name = &captures[1];
            info!("Right: {:?}", &captures[2]);
            info!("Left: {:?}", name);

            let replacement = match &captures[2] {
                "Any" => format!("(?<{name}>(?:(?s).)*?)"),
                "Heading" => format!("(?<{name}>(?:.)*?)"),
                "Regex" => format!("(?<{name}>(?:{}))", &captures[3]),
                _ => panic!("Unknown shortcut."),
            };

            regex.push_str(&replacement);
        }
        regex.push_str(better_regex);

        info!("{:?}", regex);

        Matcher {
            regex: Regex::new(&regex).unwrap(),
            end_index,
        }
    }
}

struct Matcher<I> {
    regex: Regex,
    end_index: I,
}

impl<I> Matcher<I> {
    fn new(regex: &str, end_index: I) -> Self {
        Self {
            regex: Regex::new(&format!(r"\A{regex}")).unwrap(),
            end_index,
        }
    }
}

trait CapturesIndex {
    fn get_match<'a>(&self, captures: &Captures<'a>) -> Option<Match<'a>>;
}
impl CapturesIndex for usize {
    fn get_match<'a>(&self, captures: &Captures<'a>) -> Option<Match<'a>> {
        captures.get(*self)
    }
}
impl CapturesIndex for &str {
    fn get_match<'a>(&self, captures: &Captures<'a>) -> Option<Match<'a>> {
        captures.name(self)
    }
}

struct Parser<'a>(&'a str);

impl<'a> Parser<'a> {
    fn consume<I: CapturesIndex>(&mut self, matcher: &Matcher<I>) -> Option<Captures<'_>> {
        if let Some(captures) = matcher.regex.captures(self.0) {
            self.0 = &self.0[matcher.end_index.get_match(&captures).unwrap().end()..];
            Some(captures)
        } else {
            None
        }
    }
}

pub fn run_tests() {
    parse_file(include_str!(
        "/home/coolcatcoder/Documents/GitHub/random_notes/My Path Forward.md"
    ));
}

fn parse_file(file: &str) {
    let mut parser = Parser(file);
    let better_regex = BetterRegex::new();

    //let properties_matcher = Matcher::new("---{properties: Any}---", 0);
    let properties_matcher = Matcher::new("---((?:(?s).)*?)---", 0);
    let property_matcher = Matcher::new(
        indoc! {r"
        (?x)
        \n
        (
            (?:
                .
            )*?
        ):

        (
            (?:(?s)
                .
            )*?
        )

        (?:
            (?:
                \n
                (?:
                    .
                )*?
                :
            )|\z
        )
    "},
        2,
    );

    //let section_matcher = Matcher::new("\n# {any(), title}\n", 0);
    //let section_matcher = Matcher::new("\n# ((?:(?s).)*?)\n", 1);
    // r"\n# {title: Heading}(\n{body: Any}(\n# |\z)|\z)"
    // let section_matcher =
    //     better_regex.match_next(r"\n# {title: Heading}(\n{body: Any}(\n# |\z)|\z)", "body");
    let section_matcher = better_regex.match_next(
        // Doesn't work, because the end of body includes \n# .
        r"\n# {title: Heading}{body: Regex<(\n(.|\n)*?(\n# |\z))|\z>}",
        "body",
    );

    let properties = parser
        .consume(&properties_matcher)
        .map(|captures| {
            let mut parser = Parser(&captures[1]);
            let mut others = vec![];

            while let Some(captures) = parser.consume(&property_matcher) {
                let property = InternalProperty {
                    title: DetachedStr(file.substr_range(&captures[1]).unwrap()),
                    body: DetachedStr(file.substr_range(&captures[2]).unwrap()),
                };
                others.push(property);
            }

            Properties {
                tags: vec![],
                others,
            }
        })
        .unwrap_or_default();

    while let Some(captures) = parser.consume(&section_matcher) {
        info!("Section: {:?}", &captures["title"]);
        info!("Body: {:?}", &captures[2]);
    }

    info!("Remaining:\n{:?}", parser.0);
}

#[derive(Default)]
struct Properties {
    tags: Vec<DetachedStr>,
    others: Vec<InternalProperty>,
}

struct InternalProperty {
    title: DetachedStr,
    body: DetachedStr,
}

#[derive(Clone, Copy)]
pub struct Property<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

/// A heading and its body.
struct Section(DetachedStr);

pub struct ObsidianFile {
    file: String,
    properties: Properties,
    after_properties_but_before_the_first_section: DetachedStr,
    sections: Vec<Section>,
}

impl ObsidianFile {
    pub fn tags(&self) -> impl ExactSizeIterator<Item = &str> {
        self.properties.tags.iter().map(|tag| tag.get(&self.file))
    }

    pub fn properties(&self) -> impl ExactSizeIterator<Item = Property<'_>> {
        self.properties.others.iter().map(|property| Property {
            title: property.title.get(&self.file),
            body: property.body.get(&self.file),
        })
    }
}
