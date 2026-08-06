# LSP grammar coverage

`missing_rules.txt` is a Camxes rule-name inventory. It is not, by itself,
the implementation backlog: Camxes splits many cmavo classes into separate
`*_clause` rules, while the Rust parser intentionally handles some classes
through shared functions such as `parse_tag`, `parse_sumti`, and
`parse_selbri`.

The backlog should therefore be evaluated by observable behavior: whether an
input is accepted, whether a useful CST node is produced, and whether LSP
diagnostics and completion can use that node.

Legend: ✅ implemented, △ partial or generic handling, × not yet implemented.

| Priority | Category | Examples | Parser | CST | Diagnostics | Completion |
| --- | --- | --- | --- | --- | --- | --- |
| ★★★ | Basic bridi/sumti/selbri | `mi klama`, `lo`, `la`, `be ... bei` | ✅ | ✅ | ✅ | ✅ |
| ★★★ | Tense, space, aspect | `pu`, `ca`, `ba`, `vi`, `va`, `vu`, `ca'a`, `pu'o`, `ba'o` | △ | △ | △ | △ |
| ★★★ | Quote and quotation terminators | `zo`, `lu ... li'u`, `lo'u ... le'u` | △ | ✅ | △ | △ |
| ★★★ | Common terminators | `ku`, `kei`, `do'u`, `ku'o`, `tu'u` | △ | △ | △ | × |
| ★★☆ | Relative clauses and free modifiers | `poi`, `noi`, `voi`, `soi`, `doi` | △ | △ | △ | △ |
| ★★☆ | BAI/FIhO/JAI tags | `bai`, `bau`, `fi'o`, `fe'e`, `jai` | △ | △ | △ | △ |
| ★★☆ | MEX | `li`, `tei ... foi`, `nu'a`, `moi`, `mai` | △ | △ | △ | × |
| ★☆☆ | Editing/correction | `sa`, `si`, `su`, `faho` | △ | ✅ | △ | △ |
| ★☆☆ | Rare structural/terminator variants | `KEI`, `GEhU`, `TEhU`, `TOI`, `TUhU` | △ | △ | △ | × |

## Recommended implementation order

1. Add representative tests for quote forms, common tense/space/aspect tags,
   and ordinary terminators. Measure acceptance first.
2. Give each accepted category a dedicated CST node or typed role where the
   current generic node is insufficient.
3. Extend diagnostics and recovery around the category's open/close tokens.
4. Add completion only after the parser can identify the category and its
   valid continuation points.
5. Implement MEX and correction forms after the high-frequency sentence
   structures are stable.

This keeps Camxes compatibility as the reference while making the LSP
backlog driven by user-visible behavior rather than by PEG rule count.
