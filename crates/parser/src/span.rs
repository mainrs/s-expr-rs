use auto_ops::impl_op_ex;
use chumsky::Span as SpanT;
use std::ops::Range;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Span(Range<usize>);

impl SpanT for Span {
    type Context = ();
    type Offset = usize;

    fn new((): Self::Context, range: Range<Self::Offset>) -> Self {
        Span(range)
    }
    fn context(&self) -> Self::Context {}
    fn start(&self) -> Self::Offset {
        self.0.start
    }
    fn end(&self) -> Self::Offset {
        self.0.end
    }
}

impl_op_ex!(+ |a: &Span, b: &Span| -> Span { 
    let range = Range { 
        start: a.0.start.min(b.0.start),
        end: a.0.end.min(b.0.end),
    };
    Span(range)
});

#[derive(Clone, Debug, PartialEq)]
pub struct Spanned<T> {
    pub node: T,
    pub span: Span,
}

impl<T> Spanned<T> {
    pub fn new(node: T, span: Span) -> Self {
        Self { node, span }
    }
}
