use aho_corasick::{AhoCorasick, AhoCorasickBuilder};

use crate::bot::MessageHandlerType;

pub(crate) struct KeywordRuleBuilder {
    keywords: Vec<&'static str>,
    handlers: Vec<MessageHandlerType>,
}

impl KeywordRuleBuilder {
    pub(crate) fn new() -> Self {
        KeywordRuleBuilder {
            keywords: Vec::new(),
            handlers: Vec::new(),
        }
    }
    pub(crate) fn insert(&mut self, keyword: &'static str, handler: MessageHandlerType) {
        self.keywords.push(keyword);
        self.handlers.push(handler);
    }
    pub(crate) fn build(self) -> KeywordRule {
        KeywordRule {
            handlers: self.handlers,
            matcher: AhoCorasickBuilder::new().dfa(true).build(&self.keywords),
        }
    }
}

pub(crate) struct KeywordRule {
    matcher: AhoCorasick,
    handlers: Vec<MessageHandlerType>,
}

impl KeywordRule {
    pub(crate) fn find(&self, keyword: &str) -> Option<&MessageHandlerType> {
        //TODO: match multi pattern
        self.matcher.find(keyword).map(|r| &self.handlers[r.pattern()])
    }
}
