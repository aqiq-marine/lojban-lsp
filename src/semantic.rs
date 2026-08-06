//! Lightweight semantic projection used by editor features.

use crate::{
    ast::{self, Node},
    syntax::SyntaxKind,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NodeKind {
    Sentence,
    Sumti,
    Selbri,
    Brivla,
    Cmavo,
    PlaceTag,
    Indicator,
    Vocative,
    Error,
    Other(SyntaxKind),
}

#[derive(Clone, Debug)]
pub struct SemanticNode {
    pub kind: NodeKind,
    pub node: Node,
}

#[derive(Clone, Debug)]
pub struct SemanticModel {
    root: Node,
}

impl SemanticModel {
    pub fn new(root: Node) -> Self {
        Self { root }
    }
    pub fn root(&self) -> &Node {
        &self.root
    }
    pub fn nodes(&self) -> impl Iterator<Item = SemanticNode> + '_ {
        self.root.descendants().map(|node| SemanticNode {
            kind: classify(node.kind()),
            node,
        })
    }
    pub fn node_at(&self, offset: u32) -> Option<SemanticNode> {
        let token = ast::token_at(&self.root, offset)?;
        token.parent().map(|node| SemanticNode {
            kind: classify(node.kind()),
            node,
        })
    }
}

fn classify(kind: SyntaxKind) -> NodeKind {
    match kind {
        SyntaxKind::Sentence => NodeKind::Sentence,
        SyntaxKind::Sumti => NodeKind::Sumti,
        SyntaxKind::Selbri => NodeKind::Selbri,
        SyntaxKind::Brivla => NodeKind::Brivla,
        SyntaxKind::Cmavo => NodeKind::Cmavo,
        SyntaxKind::PlaceTag => NodeKind::PlaceTag,
        SyntaxKind::Indicator => NodeKind::Indicator,
        SyntaxKind::Vocative => NodeKind::Vocative,
        SyntaxKind::Error => NodeKind::Error,
        other => NodeKind::Other(other),
    }
}
