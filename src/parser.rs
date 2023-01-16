use chumsky::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Token {
    Select,
    From,
    Group,
    Order,
    By,
    Where,
    Ident(String),
}

pub type Span = std::ops::Range<usize>;

pub fn parser() -> impl Parser<char, Vec<(Token, Span)>, Error = Simple<char>> {
    let ident = text::ident().map(|ident: String| match ident.to_lowercase().as_str() {
        "select" => Token::Select,
        "from" => Token::From,
        "where" => Token::Where,
        "group" => Token::Group,
        "order" => Token::Order,
        "by" => Token::By,
        _ => Token::Ident(ident),
    });

    let token = ident.recover_with(skip_then_retry_until([]));
    let comment = just("--").then(take_until(just('\n'))).padded();

    token
        .map_with_span(|tok, span| (tok, span))
        .padded_by(comment.repeated())
        .padded()
        .repeated()
}
