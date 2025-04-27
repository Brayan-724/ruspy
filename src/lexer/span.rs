#[derive(Clone, Copy)]
pub struct Span {
    pub line: usize,
    pub col: usize,
    pub offset: usize,
}

#[derive(Clone, Copy)]
pub struct SpanRange {
    pub from: Span,
    pub to: Span,
}

impl Span {
    pub fn zeroed() -> Self {
        Self {
            line: 0,
            col: 0,
            offset: 0,
        }
    }

    pub fn range_to(self, span: Span) -> SpanRange {
        if span.offset > self.offset {
            SpanRange {
                from: self,
                to: span,
            }
        } else {
            SpanRange {
                from: span,
                to: self,
            }
        }
    }
}
