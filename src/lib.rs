use regex::Regex;

pub trait MatcherTrait {
    fn execute(&self, line: &str) -> bool;
}

pub struct FixedStringsMatcher {
    pattern: String,
}
impl FixedStringsMatcher {
    pub fn new(pattern: String) -> FixedStringsMatcher {
        FixedStringsMatcher { pattern: pattern }
    }
}
impl MatcherTrait for FixedStringsMatcher {
    fn execute(&self, line: &str) -> bool {
        line.contains(&self.pattern)
    }
}

pub struct ExtendedRegexpMatcher {
    pattern: Regex,
}
impl ExtendedRegexpMatcher {
    pub fn new(pattern: String) -> ExtendedRegexpMatcher {
        ExtendedRegexpMatcher {
            pattern: Regex::new(&pattern).unwrap(),
        }
    }
}
impl MatcherTrait for ExtendedRegexpMatcher {
    fn execute(&self, line: &str) -> bool {
        self.pattern.is_match(line)
    }
}

pub enum Matcher {
    ExtendedRegex(ExtendedRegexpMatcher),
    FixedStrings(FixedStringsMatcher),
}
impl Matcher {
    pub fn new(pattern: String, is_fixed_strings_mode: bool) -> Matcher {
        if is_fixed_strings_mode {
            Matcher::FixedStrings(FixedStringsMatcher::new(pattern.to_string()))
        } else {
            Matcher::ExtendedRegex(ExtendedRegexpMatcher::new(pattern.to_string()))
        }
    }
    pub fn execute(&self, line: &str) -> bool {
        match self {
            Matcher::FixedStrings(m) => m.execute(line),
            Matcher::ExtendedRegex(m) => m.execute(line),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extended_regexp_matcher() {
        let matcher = Matcher::new("Z".to_string(), false);
        assert_eq!(false, matcher.execute("abcdefg"));
        let matcher = Matcher::new("a+.b+".to_string(), false);
        assert_eq!(true, matcher.execute("aaa bbb"));
    }

    #[test]
    fn test_fixed_strings_matcher() {
        let matcher = Matcher::new("fg".to_string(), true);
        assert_eq!(true, matcher.execute("abcdefg"));
        let matcher = Matcher::new("Z".to_string(), true);
        assert_eq!(false, matcher.execute("abcdefg"));
        let matcher = Matcher::new("a+.b+".to_string(), true);
        assert_eq!(false, matcher.execute("aaa bbb"));
    }
}
