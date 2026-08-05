use rowan::GreenNodeBuilder;
use crate::syntax::SyntaxKind;
use crate::ast::LojbanAST;

pub struct LojbanBuilder {
    builder: GreenNodeBuilder<'static>,
}

impl LojbanBuilder {
    pub fn new() -> Self {
        Self {
            builder: GreenNodeBuilder::new(),
        }
    }

    pub fn start_node(&mut self, kind: SyntaxKind) {
        self.builder.start_node(rowan::SyntaxKind(kind as u16));
    }

    pub fn finish_node(&mut self) {
        self.builder.finish_node();
    }

    pub fn token(&mut self, kind: SyntaxKind, text: &str) {
        self.builder.token(rowan::SyntaxKind(kind as u16), text);
    }

    pub fn finish(self) -> rowan::GreenNode {
        self.builder.finish()
    }
}

pub fn convert_to_green_node(ast: LojbanAST) -> rowan::GreenNode {
    let mut builder = LojbanBuilder::new();
    convert_recursive(ast, &mut builder);
    builder.finish()
}

fn convert_recursive(node: LojbanAST, builder: &mut LojbanBuilder) {
    match node {
        LojbanAST::Text(children) => {
            builder.start_node(SyntaxKind::Text);
            for child in children { convert_recursive(child, builder); }
            builder.finish_node();
        }
        LojbanAST::Paragraph(children) => {
            builder.start_node(SyntaxKind::Paragraph);
            for child in children { convert_recursive(child, builder); }
            builder.finish_node();
        }
        LojbanAST::Statement(children) => {
            builder.start_node(SyntaxKind::Statement);
            for child in children { convert_recursive(child, builder); }
            builder.finish_node();
        }
        LojbanAST::Sentence(children) => {
            builder.start_node(SyntaxKind::Sentence);
            for child in children { convert_recursive(child, builder); }
            builder.finish_node();
        }
        LojbanAST::Syllable(children) => {
            builder.start_node(SyntaxKind::Syllable);
            for child in children { convert_recursive(child, builder); }
            builder.finish_node();
        }
        LojbanAST::ConsonantCluster(children) => {
            builder.start_node(SyntaxKind::ConsonantCluster);
            for child in children { convert_recursive(child, builder); }
            builder.finish_node();
        }
        LojbanAST::Sumti(children) => {
            builder.start_node(SyntaxKind::Sumti);
            for child in children { convert_recursive(child, builder); }
            builder.finish_node();
        }
        LojbanAST::Selbri(children) => {
            builder.start_node(SyntaxKind::Selbri);
            for child in children { convert_recursive(child, builder); }
            builder.finish_node();
        }
        LojbanAST::Brivla(children) => {
            builder.start_node(SyntaxKind::Brivla);
            for child in children { convert_recursive(child, builder); }
            builder.finish_node();
        }
        LojbanAST::Gismu(children) => {
            builder.start_node(SyntaxKind::Gismu);
            for child in children { convert_recursive(child, builder); }
            builder.finish_node();
        }
        LojbanAST::Lujvo(children) => {
            builder.start_node(SyntaxKind::Lujvo);
            for child in children { convert_recursive(child, builder); }
            builder.finish_node();
        }
        LojbanAST::Fuhivla(children) => {
            builder.start_node(SyntaxKind::Fuhivla);
            for child in children { convert_recursive(child, builder); }
            builder.finish_node();
        }
        LojbanAST::Negation(children) => {
            builder.start_node(SyntaxKind::Negation);
            for child in children { convert_recursive(child, builder); }
            builder.finish_node();
        }
        LojbanAST::Tag(children) => {
            builder.start_node(SyntaxKind::Tag);
            for child in children { convert_recursive(child, builder); }
            builder.finish_node();
        }
        LojbanAST::Quoting(children) => {
            builder.start_node(SyntaxKind::Quoting);
            for child in children { convert_recursive(child, builder); }
            builder.finish_node();
        }
        LojbanAST::Abstractor(children) => {
            builder.start_node(SyntaxKind::Abstractor);
            for child in children { convert_recursive(child, builder); }
            builder.finish_node();
        }
        LojbanAST::Cmavo(_kind, children) => {
            builder.start_node(SyntaxKind::Cmavo);
            // We could use `_kind` to add metadata here if needed
            for child in children { convert_recursive(child, builder); }
            builder.finish_node();
        }
        LojbanAST::FreeModifier(children) => {
            builder.start_node(SyntaxKind::FreeModifier);
            for child in children { convert_recursive(child, builder); }
            builder.finish_node();
        }
        LojbanAST::Token(text) => {
            builder.token(SyntaxKind::Whitespace, &text);
        }
        LojbanAST::Cmene(text) => {
            builder.start_node(SyntaxKind::Cmene);
            builder.token(SyntaxKind::Whitespace, &text);
            builder.finish_node();
        }
        LojbanAST::Prenex(children) => {
            builder.start_node(SyntaxKind::Prenex);
            for child in children { convert_recursive(child, builder); }
            builder.finish_node();
        }
        LojbanAST::Bridi(children) => {
            builder.start_node(SyntaxKind::Bridi);
            for child in children { convert_recursive(child, builder); }
            builder.finish_node();
        }
    }
}
