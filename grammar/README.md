# Grammar source of truth

The standard grammar is `../src-js/ilmentufa/camxes.peg`.

This directory is intentionally not a second copy of the grammar. Rust
parser rules must be migrated from that file, and every migrated rule should
retain the Camxes rule name until the conformance tests for that rule exist.
The other Ilmentufa grammars (`camxes-beta.peg`, `camxes-beta-cbm.peg`, and
`camxes-beta-cbm-ckt.peg`) are variants, not replacements for the standard
grammar.

Migration policy:

1. Keep lexical/morphological rules separate from syntactic rules.
2. Preserve trivia and source ranges in the CST.
3. Build the Green Tree during parsing; do not create an intermediate owned
   AST in PEG actions.
4. Compare normalized Rust and Camxes trees in conformance tests.
5. Allow `Error` and `Missing` nodes for incomplete LSP input.
