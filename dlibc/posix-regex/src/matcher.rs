//! The matcher: Can find substrings in a string that match any compiled regex

#[cfg(feature = "no_std")]
use std::prelude::*;

use compile::{Range, Token};
use ctype;
use std::borrow::Cow;
use std::fmt;
use std::rc::Rc;

/// A regex matcher, ready to match stuff
#[derive(Clone)]
pub struct PosixRegex<'a> {
    branches: Cow<'a, [Vec<(Token, Range)>]>,
    case_insensitive: bool,
    newline: bool,
    no_start: bool,
    no_end: bool,
}
impl<'a> PosixRegex<'a> {
    /// Create a new matcher instance from the specified alternations. This
    /// should probably not be used and instead an instance should be obtained
    /// from `PosixRegexBuilder`, which also compiles a string into regex.
    pub fn new(branches: Cow<'a, [Vec<(Token, Range)>]>) -> Self {
        Self {
            branches,
            case_insensitive: false,
            newline: false,
            no_start: false,
            no_end: false,
        }
    }
    /// Chainable function to enable/disable case insensitivity. Default: false.
    /// When enabled, single characters match both their upper and lowercase
    /// representations.
    pub fn case_insensitive(mut self, value: bool) -> Self {
        self.case_insensitive = value;
        self
    }
    /// Chainable function to enable/disable newline mode. Default: false.
    /// When enabled, ^ and $ match newlines as well as start/end.
    /// This behavior overrides both no_start and no_end.
    pub fn newline(mut self, value: bool) -> Self {
        self.newline = value;
        self
    }
    /// Chainable function to enable/disable no_start mode. Default: false.
    /// When enabled, ^ doesn't actually match the start of a string.
    pub fn no_start(mut self, value: bool) -> Self {
        self.no_start = value;
        self
    }
    /// Chainable function to enable/disable no_start mode. Default: false.
    /// When enabled, $ doesn't actually match the end of a string.
    pub fn no_end(mut self, value: bool) -> Self {
        self.no_end = value;
        self
    }
    /// Return the total number of matches that **will** be returned by
    /// `matches_exact` or in each match in `matches`.
    pub fn count_groups(&self) -> usize {
        let mut count = 1;
        for branch in &*self.branches {
            count += count_groups(branch);
        }
        count
    }
    /// Match the string starting at the current position. This does not find
    /// substrings.
    pub fn matches_exact(&self, input: &[u8]) -> Option<Box<[Option<(usize, usize)>]>> {
        let mut matcher = PosixRegexMatcher {
            base: self,
            input,
            offset: 0,
        };
        let branches = self
            .branches
            .iter()
            .filter_map(|tokens| Branch::new(true, tokens))
            .collect();

        let start = matcher.offset;
        match matcher.matches_exact(branches) {
            None => None,
            Some(mut groups) => {
                assert_eq!(groups[0], None);
                groups[0] = Some((start, matcher.offset));
                Some(groups)
            }
        }
    }
    /// Match any substrings in the string, but optionally no more than `max`
    pub fn matches(
        &self,
        input: &[u8],
        mut max: Option<usize>,
    ) -> Vec<Box<[Option<(usize, usize)>]>> {
        let mut matcher = PosixRegexMatcher {
            base: self,
            input,
            offset: 0,
        };

        let tokens = vec![
            (Token::InternalStart, Range(0, None)),
            (
                Token::Group {
                    id: 0,
                    branches: self.branches.to_vec(),
                },
                Range(1, Some(1)),
            ),
        ];
        let branches = vec![Branch::new(false, &tokens).unwrap()];

        let mut matches = Vec::new();
        while max.map(|max| max > 0).unwrap_or(true) {
            match matcher.matches_exact(branches.clone()) {
                Some(groups) => matches.push(groups),
                None => break,
            }
            max = max.map(|max| max - 1);
        }
        matches
    }
}

fn count_groups(tokens: &[(Token, Range)]) -> usize {
    let mut groups = 0;
    for (token, _) in tokens {
        if let Token::Group { ref branches, .. } = token {
            groups += 1;
            for branch in branches {
                groups += count_groups(branch);
            }
        }
    }
    groups
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Group {
    index: usize,
    variant: usize,
    id: usize,
}

#[derive(Clone)]
struct Branch<'a> {
    index: usize,
    repeated: u32,
    tokens: &'a [(Token, Range)],
    path: Box<[Group]>,
    prev: Box<[Option<(usize, usize)>]>,

    parent: Option<Rc<Branch<'a>>>,
}
impl<'a> fmt::Debug for Branch<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (ref token, mut range) = *self.get_token();
        range.0 = range.0.saturating_sub(self.repeated);
        range.1 = range.1.map(|max| max.saturating_sub(self.repeated));
        write!(f, "{:?}", (token, range))
    }
}
impl<'a> Branch<'a> {
    fn new(exact: bool, tokens: &'a [(Token, Range)]) -> Option<Self> {
        if tokens.is_empty() {
            return None;
        }
        Some(Self {
            index: 0,
            repeated: 0,
            tokens: tokens,
            path: Box::new([]),
            prev: vec![None; if exact { 1 } else { 0 } + count_groups(tokens)].into_boxed_slice(),

            parent: None,
        })
    }
    fn group(
        path: Box<[Group]>,
        prev: Box<[Option<(usize, usize)>]>,
        tokens: &'a [(Token, Range)],
        mut parent: Branch<'a>,
    ) -> Option<Self> {
        if tokens.is_empty() {
            return None;
        }
        parent.repeated += 1;
        Some(Self {
            index: 0,
            repeated: 0,
            tokens,
            path,
            prev,
            parent: Some(Rc::new(parent)),
        })
    }
    fn parent_tokens(&self) -> &[(Token, Range)] {
        let mut tokens = self.tokens;

        let len = self.path.len();
        if len > 0 {
            for group in &self.path[..len - 1] {
                match tokens[group.index] {
                    (Token::Group { ref branches, .. }, _) => tokens = &branches[group.variant],
                    _ => panic!("non-group index in path"),
                }
            }
        }

        tokens
    }
    fn tokens(&self) -> &[(Token, Range)] {
        let mut tokens = self.parent_tokens();

        if let Some(group) = self.path.last() {
            match tokens[group.index] {
                (Token::Group { ref branches, .. }, _) => tokens = &branches[group.variant],
                _ => panic!("non-group index in path"),
            }
        }

        tokens
    }
    fn get_token(&self) -> &(Token, Range) {
        &self.tokens()[self.index]
    }
    fn update_group_end(&mut self, offset: usize) {
        for group in &mut *self.path {
            self.prev[group.id].as_mut().unwrap().1 = offset;
        }
    }
    fn extend(&self, prev: &mut Box<[Option<(usize, usize)>]>) {
        for (i, &group) in self.prev.iter().enumerate() {
            if group.is_some() {
                prev[i] = group;
            }
        }
    }
    fn next_branch(&self) -> Option<Self> {
        if self.index + 1 >= self.tokens().len() {
            let parent = self.parent.as_ref()?;
            let (_, Range(min, _)) = *parent.get_token();
            // Don't add the next branch until we've repeated this one enough
            if parent.repeated < min {
                return None;
            }

            if let Some(mut next) = parent.next_branch() {
                // Group is closing, migrate previous & current groups to next.
                self.extend(&mut next.prev);

                return Some(next);
            }
            return None;
        }
        Some(Self {
            index: self.index + 1,
            repeated: 0,
            ..self.clone()
        })
    }
    fn add_repeats(&self, branches: &mut Vec<Branch<'a>>, offset: usize) {
        let mut branch = self;
        loop {
            if let (
                Token::Group {
                    id,
                    branches: ref alternatives,
                },
                Range(_, max),
            ) = *branch.get_token()
            {
                if max.map(|max| branch.repeated < max).unwrap_or(true) {
                    for alternative in 0..alternatives.len() {
                        let mut path = branch.path.to_vec();
                        path.push(Group {
                            variant: alternative,
                            index: branch.index,
                            id,
                        });

                        let mut prev = self.prev.clone();
                        prev[id].get_or_insert((0, 0)).0 = offset;

                        if let Some(group) = Branch::group(
                            path.into_boxed_slice(),
                            prev,
                            branch.tokens,
                            branch.clone(),
                        ) {
                            branches.push(group);
                        }
                    }
                    break;
                }
            }

            match branch.parent {
                Some(ref new) => branch = new,
                None => break,
            }
        }
    }
    /// Returns if this node is "explored" enough times,
    /// meaning it has repeated as many times as it want to and has nowhere to go next.
    fn is_explored(&self) -> bool {
        let mut branch = Cow::Borrowed(self);

        loop {
            {
                let mut branch = &*branch;
                while let Some(ref parent) = branch.parent {
                    let (_, Range(min, _)) = *parent.get_token();
                    if parent.repeated < min {
                        // Group did not repeat enough times!
                        return false;
                    }
                    branch = parent;
                }
            }

            let (_, Range(min, _)) = *branch.get_token();
            if branch.repeated < min {
                return false;
            }
            match branch.next_branch() {
                Some(next) => branch = Cow::Owned(next),
                None => break,
            }
        }
        true
    }
}

struct PosixRegexMatcher<'a> {
    base: &'a PosixRegex<'a>,
    input: &'a [u8],
    offset: usize,
}
impl<'a> PosixRegexMatcher<'a> {
    fn expand<'b>(&mut self, branches: &mut [Branch<'b>]) -> Vec<Branch<'b>> {
        let mut insert = Vec::new();

        for branch in branches {
            branch.update_group_end(self.offset);

            let (ref token, range) = *branch.get_token();

            if let Token::Group {
                id,
                branches: ref inner,
            } = *token
            {
                for alternation in 0..inner.len() {
                    let mut path = Vec::with_capacity(branch.path.len() + 1);
                    path.extend_from_slice(&branch.path);
                    path.push(Group {
                        index: branch.index,
                        variant: alternation,
                        id,
                    });

                    let mut prev = branch.prev.clone();
                    prev[id].get_or_insert((0, 0)).0 = self.offset;

                    if let Some(branch) =
                        Branch::group(path.into(), prev, branch.tokens, branch.clone())
                    {
                        insert.push(branch);
                    }
                }
            }
            if branch.repeated >= range.0 {
                // Push the next element as a new branch
                if let Some(next) = branch.next_branch() {
                    insert.push(next);
                }
                branch.add_repeats(&mut insert, self.offset);
            }
        }

        if !insert.is_empty() {
            let mut new = self.expand(&mut insert);
            insert.append(&mut new);
        }
        insert
    }

    fn matches_exact(
        &mut self,
        mut branches: Vec<Branch>,
    ) -> Option<Box<[Option<(usize, usize)>]>> {
        // Whether or not any branch, at any point, got fully explored. This
        // means at least one path of the regex successfully completed!
        let mut succeeded = None;
        let mut prev = self
            .offset
            .checked_sub(1)
            .and_then(|index| self.input.get(index).cloned());

        loop {
            let next = self.input.get(self.offset).cloned();

            let mut index = 0;
            let mut remove = 0;

            let mut insert = self.expand(&mut branches);
            branches.append(&mut insert);

            loop {
                if index >= branches.len() {
                    break;
                }
                if remove > 0 {
                    // Just like Rust's `retain` function, shift all elements I
                    // want to keep back and `truncate` when I'm done.
                    branches.swap(index, index - remove);
                }
                let branch = &mut branches[index - remove];
                index += 1;

                let (ref token, Range(_, mut max)) = *branch.get_token();
                let mut token = token;

                let mut accepts = true;

                // Step 1: Handle zero-width stuff like ^ and \<
                loop {
                    match token {
                        Token::End | Token::Start | Token::WordEnd | Token::WordStart => {
                            accepts = accepts
                                && match token {
                                    Token::End => {
                                        (!self.base.no_end && next.is_none())
                                            || (self.base.newline && next == Some(b'\n'))
                                    }
                                    Token::Start => {
                                        (!self.base.no_start && self.offset == 0)
                                            || (self.base.newline && prev == Some(b'\n'))
                                    }
                                    Token::WordEnd => {
                                        next.map(ctype::is_word_boundary).unwrap_or(true)
                                    }
                                    Token::WordStart => {
                                        prev.map(ctype::is_word_boundary).unwrap_or(true)
                                    }
                                    _ => unreachable!(),
                                };

                            // Skip ahead to the next token.
                            match branch.next_branch() {
                                Some(next) => *branch = next,
                                None => break,
                            }
                            let (ref new_token, Range(_, new_max)) = *branch.get_token();
                            token = new_token;
                            max = new_max;
                        }
                        _ => break,
                    }
                }

                // Step 2: Check if the token isn't repeated enough times already
                accepts = accepts && max.map(|max| branch.repeated < max).unwrap_or(true);

                // Step 3: Check if the token matches
                accepts = accepts
                    && match *token {
                        Token::InternalStart => next.is_some(),
                        Token::Group { .. } => false, // <- content is already expanded and handled

                        Token::Any => next
                            .map(|c| !self.base.newline || c != b'\n')
                            .unwrap_or(false),
                        Token::Char(c) => {
                            if self.base.case_insensitive {
                                next.map(|c2| c & !32 == c2 & !32).unwrap_or(false)
                            } else {
                                next == Some(c)
                            }
                        }
                        Token::OneOf { invert, ref list } => {
                            if let Some(next) = next {
                                (!invert || !self.base.newline || next != b'\n')
                                    && list
                                        .iter()
                                        .any(|c| c.matches(next, self.base.case_insensitive))
                                        == !invert
                            } else {
                                false
                            }
                        }

                        // These will only get called if they are encountered at
                        // EOF (because next_branch returns None), for example
                        // "abc\>" or "^". Then we simply want to return true as to
                        // preserve the current `accepts` status.
                        Token::End | Token::Start | Token::WordEnd | Token::WordStart => true,
                    };

                if !accepts {
                    if branch.is_explored() {
                        succeeded = Some(branch.clone());
                    }
                    remove += 1;
                    continue;
                }

                branch.repeated += 1;
            }
            let end = branches.len() - remove;
            branches.truncate(end);

            if branches.is_empty() ||
                    // The internal start thing is lazy, not greedy:
                    (succeeded.is_some() && branches.iter().all(|t| t.get_token().0 == Token::InternalStart))
            {
                return succeeded.map(|branch| branch.prev);
            }

            if next.is_some() {
                self.offset += 1;
                prev = next;
            }
        }
    }
}
