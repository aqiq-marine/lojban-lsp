//! Typed, zero-copy views over the Rowan syntax tree.

use rowan::{SyntaxElement, SyntaxNode, SyntaxToken, TextRange};

use crate::syntax::{LojbanLanguage, SyntaxKind};

pub type Node = SyntaxNode<LojbanLanguage>;
pub type Token = SyntaxToken<LojbanLanguage>;

pub trait AstNode: Sized + Clone {
    fn cast(node: Node) -> Option<Self>;
    fn syntax(&self) -> &Node;
    fn kind(&self) -> SyntaxKind {
        self.syntax().kind()
    }
    fn text(&self) -> String {
        self.syntax().text().to_string()
    }
    fn range(&self) -> TextRange {
        self.syntax().text_range()
    }
    fn tokens(&self) -> impl Iterator<Item = Token> {
        self.syntax()
            .descendants_with_tokens()
            .filter_map(|e| e.into_token())
    }
}

macro_rules! node_views {
    ($($name:ident),+ $(,)?) => {
        $(
            #[derive(Clone, Debug)]
            pub struct $name(pub Node);
            impl AstNode for $name {
                fn cast(node: Node) -> Option<Self> {
                    (node.kind() == SyntaxKind::$name).then_some(Self(node))
                }
                fn syntax(&self) -> &Node { &self.0 }
            }
        )+
    };
}

node_views!(
    Text, Sentence, Bridi, Sumti, Selbri, Terms, Tanru, PlaceTag, Indicator, Vocative, Brivla,
    Cmavo, Error, Mex, Operand
);

impl Text {
    pub fn sentences(&self) -> impl Iterator<Item = Sentence> + '_ {
        self.0.descendants().filter_map(Sentence::cast)
    }
    pub fn errors(&self) -> impl Iterator<Item = Error> + '_ {
        self.0.descendants().filter_map(Error::cast)
    }
}

impl Sentence {
    pub fn bridi(&self) -> Option<Bridi> {
        child(self, SyntaxKind::Bridi)
    }
    pub fn sumti(&self) -> impl Iterator<Item = Sumti> + '_ {
        descendants(self, SyntaxKind::Sumti)
    }
    pub fn selbri(&self) -> Option<Selbri> {
        child(self, SyntaxKind::Selbri)
    }
}

impl Bridi {
    pub fn terms(&self) -> Option<Terms> {
        child(self, SyntaxKind::Terms)
    }
    pub fn selbri(&self) -> Option<Selbri> {
        child(self, SyntaxKind::Selbri)
    }
    pub fn sumti(&self) -> impl Iterator<Item = Sumti> + '_ {
        descendants(self, SyntaxKind::Sumti)
    }
}

impl Sumti {
    pub fn relative_clauses(&self) -> impl Iterator<Item = SyntaxNode<LojbanLanguage>> + '_ {
        self.0
            .children()
            .filter(|n| n.kind() == SyntaxKind::RelativeClause)
    }
}

impl Selbri {
    pub fn tanru(&self) -> impl Iterator<Item = Tanru> + '_ {
        descendants(self, SyntaxKind::Tanru)
    }
    pub fn brivla(&self) -> impl Iterator<Item = Brivla> + '_ {
        descendants(self, SyntaxKind::Brivla)
    }
    pub fn cmavo(&self) -> impl Iterator<Item = Cmavo> + '_ {
        descendants(self, SyntaxKind::Cmavo)
    }
}

impl Tanru {
    pub fn brivla(&self) -> impl Iterator<Item = Brivla> + '_ {
        descendants(self, SyntaxKind::Brivla)
    }
}

impl Bridi {
    pub fn place_tags(&self) -> impl Iterator<Item = PlaceTag> + '_ {
        descendants(self, SyntaxKind::PlaceTag)
    }
}

fn child<T: AstNode>(owner: &impl AstNode, kind: SyntaxKind) -> Option<T> {
    owner
        .syntax()
        .children()
        .find_map(|n| (n.kind() == kind).then(|| T::cast(n)).flatten())
}
fn descendants<'a, T: AstNode + 'a>(
    owner: &'a impl AstNode,
    kind: SyntaxKind,
) -> impl Iterator<Item = T> + 'a {
    owner
        .syntax()
        .descendants()
        .filter(move |n| n.kind() == kind)
        .filter_map(T::cast)
}

pub fn token_at(root: &Node, offset: u32) -> Option<Token> {
    root.token_at_offset(rowan::TextSize::from(offset))
        .right_biased()
        .or_else(|| {
            root.token_at_offset(rowan::TextSize::from(offset))
                .left_biased()
        })
}

pub fn element_parent(element: &SyntaxElement<LojbanLanguage>) -> Option<Node> {
    element.parent()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ParserOptions, parse};

    #[test]
    fn typed_views_traverse_parser_tree() {
        let parse = parse("mi klama", ParserOptions::default());
        let root = parse.ast();
        let sentence = root.sentences().next().expect("sentence");
        assert!(sentence.bridi().is_some());
        assert!(sentence.sumti().next().is_some());
        assert_eq!(root.text(), "mi klama");
    }
}
