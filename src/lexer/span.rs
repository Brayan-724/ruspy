use std::ops::Range;

#[derive(Clone, Copy, Debug)]
pub struct Span {
    pub from: usize,
    pub to: usize,
}

pub trait IntoSpan<V> {
    fn into_span(self) -> V;
}

impl ariadne::Span for Span {
    type SourceId = ();

    fn source(&self) -> &Self::SourceId {
        &()
    }

    fn start(&self) -> usize {
        self.from
    }

    fn end(&self) -> usize {
        self.to
    }
}

impl From<Range<usize>> for Span {
    fn from(value: Range<usize>) -> Self {
        Self {
            from: value.start,
            to: value.end,
        }
    }
}

impl Span {
    pub const ZERO: Self = Self { from: 0, to: 0 };

    pub const fn char(offset: usize) -> Self {
        Self {
            from: offset,
            to: offset + 1,
        }
    }
}

impl<T> IntoSpan<(T, Span)> for (T, Range<usize>) {
    fn into_span(self) -> (T, Span) {
        (self.0, Span::from(self.1))
    }
}
