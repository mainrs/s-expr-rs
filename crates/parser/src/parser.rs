use chumsky::prelude::*;
use crate::{span::{Span, Spanned}, ast::AstNode};

pub type ParserError = Simple<char, Span>;

fn symbol() -> impl Parser<char, Spanned<AstNode>, Error = ParserError> {
    filter(|c: &char| c.is_ascii_alphanumeric() || *c == '-')
        .repeated()
        .at_least(1)
        .collect::<String>()
        .map(AstNode::Symbol)
        .map_with_span(Spanned::new)
        .labelled("symbol")
}

fn integer() -> impl Parser<char, Spanned<AstNode>, Error = ParserError> {
    text::int(10)
        .repeated()
        .at_least(1)
        .collect::<String>()
        .from_str()
        .unwrapped()
        .map(AstNode::Integer)
        .map_with_span(Spanned::new)
        .labelled("i64")
}

fn string() -> impl Parser<char, Spanned<AstNode>, Error = ParserError> {
    just('"')
        .ignore_then(filter(|c| *c != '"').repeated())
        .then_ignore(just('"'))
        .collect::<String>()
        .map(AstNode::String)
        .map_with_span(Spanned::new)
        .labelled("string")
}

fn bool() -> impl Parser<char, Spanned<AstNode>, Error = ParserError> {
    let r#true = just("true").map(|_| AstNode::Bool(true));
    let r#false = just("false").map(|_| AstNode::Bool(false));

    r#true.or(r#false).map_with_span(Spanned::new).labelled("bool")
}

fn list() -> impl Parser<char, Spanned<AstNode>, Error = ParserError> {   
    recursive::<char, Spanned<AstNode>, _, _, ParserError>(|list: Recursive<_, Spanned<AstNode>, _>| {
        list.padded()
            .repeated()
            .map(|exprs| {
                let (_items, spans): (Vec<_>, Vec<_>) = exprs.clone()
                    .into_iter()
                    .map(|spanned| (spanned.node, spanned.span))
                    .unzip();
                
                let span = spans.into_iter().reduce(|init, item| {
                    init + item
                });
                // let (items, spans): (Vec<_>, Vec<_>) = exprs.clone().into_iter()
                //     .map(|spanned| (spanned.0, spanned.1))
                //     .unzip();
                // let span = spans.iter().fold(0..0, |acc, item| {
                //     acc.start.min(item.start)..acc.end.max(item.end)
                // });
                Spanned::new(AstNode::List(exprs), span.unwrap())
            })
            .delimited_by(just('('), just(')'))
            // .recover_with(nested_delimiters('(', ')', [], |span| Spanned(AstNode::Error, span)))
            // .or(integer())
            // .or(bool())
            // .or(symbol())
            // .or(string())
    })
}

pub fn parser() -> impl Parser<char, Vec<Spanned<AstNode>>, Error = ParserError> {
    let comment = just(";")
        .then(take_until(just('\n')))
        .padded()
        .repeated();

    list()
        .padded_by(comment)
        .padded()
        .repeated()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symbol() {
        let i = "hello-world";
        let res = symbol().parse(i);
        // assert_eq!(res.unwrap().node, AstNode::Symbol(i.to_owned()));
    }

    #[test]
    fn test_list_recovery() {
        let i = "(a1 b (c d) (e))\n(define (a 1))";
        let res = parser().parse_recovery(i);
        println!("{:#?}", res);
    }
}

