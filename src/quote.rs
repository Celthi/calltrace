pub trait MatchQuote {
    fn match_quote(&self, line: &str) -> bool;
}

pub struct ExactMatch {
    m: String,
}
impl ExactMatch {
    pub fn new(m: String) -> Self {
        ExactMatch { m }
    }
}
impl MatchQuote for ExactMatch {
    fn match_quote(&self, line: &str) -> bool {
        self.m == line
    }
}
pub struct PrefixMatch {
    prefix: String,
}
impl MatchQuote for PrefixMatch {
    fn match_quote(&self, line: &str) -> bool {
        line.starts_with(&self.prefix)
    }
}
impl PrefixMatch {
    pub fn new(prefix: String) -> Self {
        PrefixMatch { prefix }
    }
}
pub struct TrimPrefixMatch {
    prefix: String,
}
impl MatchQuote for TrimPrefixMatch {
    fn match_quote(&self, line: &str) -> bool {
        line.trim_start().starts_with(&self.prefix)
    }
}
impl TrimPrefixMatch {
    pub fn new(prefix: String) -> Self {
        TrimPrefixMatch {
            prefix: prefix.trim().to_string(),
        }
    }
}
pub struct TrimExactMatch {
    m: String,
}
impl TrimExactMatch {
    pub fn new(s: String) -> Self {
        TrimExactMatch {
            m: s.trim().to_string(),
        }
    }
}

impl MatchQuote for TrimExactMatch {
    fn match_quote(&self, line: &str) -> bool {
        self.m == line.trim()
    }
}

pub struct KeywordMatch {
    k: String,
}
impl KeywordMatch {
    pub fn new(k: String) -> Self {
        KeywordMatch { k }
    }
}

impl MatchQuote for KeywordMatch {
    fn match_quote(&self, line: &str) -> bool {
        line.contains(&self.k)
    }
}

pub struct ExcludeKeywordMatch {
    e: String,
}
impl ExcludeKeywordMatch {
    pub fn new(e: String) -> Self {
        ExcludeKeywordMatch { e }
    }
}
impl MatchQuote for ExcludeKeywordMatch {
    fn match_quote(&self, line: &str) -> bool {
        !line.contains(&self.e)
    }
}
