mod assignment;
mod edge;
mod node;
mod statement;
mod subgraph;

use std::marker::PhantomData;

pub use assignment::Assignment;
pub use edge::Edge;
pub use edge::{Directed, Undirected};
pub use node::Node;
pub use statement::Statement;
pub use subgraph::Subgraph;

/// An ID represents any identifier used inside 
/// of a graphviz file. This could represent an attribute
/// name, an attribute value, a graph or sugraph name, and node names 
pub type ID = String;

use crate::lex::{Peekable, PeekableLexer, Token};
use crate::parse::Constructable;

impl Constructable for ID {
    type Output = Self;
    fn from_lexer(
        mut token_stream: crate::lex::PeekableLexer,
    ) -> anyhow::Result<(Self::Output, crate::lex::PeekableLexer), anyhow::Error> {
        if let Some(Token::ID) = token_stream.next() {
            Ok((token_stream.slice().to_owned(), token_stream))
        } else {
            Err(anyhow::anyhow!("Expected type ID"))
        }
    }
}

#[derive(Debug)]
pub struct Graph<T> {
    pub id: ID,
    pub is_strict: bool,
    pub statements: Vec<Statement<T>>,
    _pd: PhantomData<T>,
}

impl Constructable for Graph<Directed> {
    type Output = Self;
    fn from_lexer(
        mut token_stream: PeekableLexer,
    ) -> anyhow::Result<(Self::Output, PeekableLexer), anyhow::Error> {
        let mut is_strict = false;
        if token_stream.peek() == Some(&Token::Strict) {
            token_stream.next();
            is_strict = true;
        }
        match token_stream.next() {
            Some(Token::Digraph) => {
                match (
                    token_stream.next(),
                    String::from(token_stream.slice()),
                    token_stream.next(),
                ) {
                    (Some(Token::ID), graph_id, Some(Token::OpenParen)) => {
                        let (statements, tstream) =
                            Vec::<Statement<Directed>>::from_lexer(token_stream)?;
                        Ok((
                            Self {
                                id: graph_id,
                                statements,
                                is_strict,
                                _pd: PhantomData,
                            },
                            tstream,
                        ))
                    }
                    _ => {
                        todo!()
                    }
                }
            }
            _ => Err(anyhow::anyhow!("Error; invalid start token")),
        }
    }
}

impl Constructable for Graph<Undirected> {
    type Output = Self;
    fn from_lexer(
        mut token_stream: PeekableLexer,
    ) -> anyhow::Result<(Self::Output, PeekableLexer), anyhow::Error> {
        let mut is_strict = false;
        if token_stream.peek() == Some(&Token::Strict) {
            token_stream.next();
            is_strict = true;
        }

        match token_stream.next() {
            Some(Token::Graph) => {
                match (
                    token_stream.next(),
                    String::from(token_stream.slice()),
                    token_stream.next(),
                ) {
                    (Some(Token::ID), graph_id, Some(Token::OpenParen)) => {
                        let (statements, tstream) =
                            Vec::<Statement<Undirected>>::from_lexer(token_stream)?;
                        Ok((
                            Self {
                                id: graph_id,
                                statements,
                                is_strict,
                                _pd: PhantomData,
                            },
                            tstream,
                        ))
                    }
                    _ => {
                        todo!()
                    }
                }
            }
            _ => Err(anyhow::anyhow!("Error; invalid start token")),
        }
    }
}
