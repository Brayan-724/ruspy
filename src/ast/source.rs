use std::collections::VecDeque;

use crate::lexer::token::SpannedToken;

pub struct SourceAst<'i> {
    pub base: &'i str,
    pub tokens: VecDeque<SpannedToken>
}
