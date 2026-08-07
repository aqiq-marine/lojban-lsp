# lojban-lsp

Lojban の [Language Server Protocol (LSP)](https://microsoft.github.io/language-server-protocol/) 実装です。テキストエディタ上でリアルタイムの構文チェック、自動補完、ホバー情報表示などを提供します。

## 機能

| 機能 | 説明 |
|---|---|
| **構文診断** | 構文エラー、不明な単語、ポーズ（`.`）の欠落警告をリアルタイムで表示 |
| **自動補完** | 文脈に応じた cmavo / brivla の候補提示（プレフィックス検索対応） |
| **ホバー情報** | 単語の品詞分類（gismu / lujvo / fuhivla / cmavo / cmevla）、selma'o、語義、lujvo 分解を表示 |
| **セマンティックトークン** | selma'o ごとに分類された 126 種類のトークンタイプによる構文ハイライト |
| **形態素解析** | gismu / cmavo / lujvo / fuhivla / cmevla の自動判定、rafsi 分割 |
| **辞書** | 1,500 語超の組み込み辞書（`dictionary.csv` からビルド時に生成） |

### 診断コード

| コード | 内容 |
|---|---|
| `LOJ000` | 構文エラー |
| `LOJ001` | 予期しないトークン |
| `LOJ002` | selbri が必要 |
| `LOJ003` | sumti が必要 |
| `LOJ004` | bridi が必要 |
| `LOJ005` | cmevla が必要 |
| `LOJ200` | 文接続詞 `.i` の前にポーズが必要（警告） |
| `LOJ201` | cmevla の前にポーズが必要（警告） |

## インストール

### LSP サーバー

```shell
cargo install --git https://github.com/aqiq-marine/lojban-lsp
```

### Neovim（lazy.nvim）

```lua
require("lazy").setup({
    spec = {
        {
            "aqiq-marine/lojban.nvim",
            config = function()
                require("lojban").setup()
            end,
        }
    },
})
```

## アーキテクチャ

```
lojban-lsp
├── src/
│   ├── main.rs            # エントリポイント（tokio ランタイム起動）
│   ├── lexer.rs           # ロスレス字句解析器
│   ├── syntax.rs          # SyntaxKind 定義（rowan 言語実装）
│   ├── cst.rs             # CST イベント・グリーンツリー構築
│   ├── cst_parser.rs      # 再帰下降構文解析器（Camxes 互換）
│   ├── ast.rs             # 型付き AST ビュー（ゼロコピー）
│   ├── morphology.rs      # 形態素解析（gismu/cmavo/lujvo/fuhivla/cmevla 判定）
│   ├── semantic.rs        # セマンティックモデル（字句分類・辞書照合）
│   ├── features.rs        # 診断生成・補完コンテキスト
│   ├── parser/            # パーサーモジュール境界
│   │   ├── mod.rs         # 公開 API (parse, ParserOptions)
│   │   ├── grammar.rs     # 文法ルール
│   │   ├── recovery.rs    # エラー回復
│   │   └── tokens.rs      # トークンユーティリティ
│   └── lsp/               # LSP プロトコル実装
│       ├── server.rs      # tower-lsp Backend 実装
│       ├── document.rs    # ドキュメント管理
│       ├── completion.rs  # 補完プロバイダー
│       ├── hover.rs       # ホバー情報
│       ├── diagnostics.rs # 診断変換
│       └── dictionary.rs  # 辞書（selma'o 全 122 種・phf ハッシュマップ）
├── dictionary.csv         # 辞書データ（ビルド時にコード生成）
├── build.rs               # dictionary.csv → phf_map! コード生成
├── grammar/               # 文法仕様ドキュメント
└── src-js/ilmentufa/      # Camxes リファレンス文法（PEG）
```

### パイプライン

```
ソーステキスト
  │
  ▼
Lexer (lex_lossless)         ← ロスレス字句解析、空白・改行・ポーズ保持
  │
  ▼
CST Parser (cst_parser)      ← 再帰下降、エラー回復、Event → GreenNode (rowan)
  │
  ▼
AST (ast.rs)                 ← 型付きゼロコピービュー (Text, Sentence, Bridi, ...)
  │
  ▼
Semantic Model (semantic.rs) ← 形態素解析 + 辞書照合 → SemanticTokenInfo
  │
  ▼
Features (features.rs)       ← Diagnostic / CompletionContext 生成
  │
  ▼
LSP Server (tower-lsp)       ← textDocument/didOpen, hover, completion, semanticTokens
```

### 主要な設計判断

- **ロスレス構文木**: [rowan](https://crates.io/crates/rowan) を使用し、空白・コメント・不正トークンを含む全バイトを CST に保持。エディタの位置情報と 1:1 で対応。
- **Camxes 互換**: PEG 文法 `camxes.peg` を参照し、ルール名を維持しながら再帰下降パーサーへ移行中。
- **コンパイル時辞書**: `build.rs` が `dictionary.csv` を `phf::phf_map!` に変換し、ランタイムコスト 0 の完全ハッシュマップを生成。
- **ポーズ診断の分離**: ポーズルールをパーサーから独立させ、補完やホバーがポーズに依存しない設計。

## 開発

### ビルド

```shell
cargo build
```

### テスト

```shell
cargo test
```

### 辞書の更新

`dictionary.csv` を編集後、再ビルドすると自動的に辞書コードが再生成されます。

CSV フォーマット:
```
word,word_kind,selmaho,brivla_kind,arity,description
```

### 文法カバレッジ

現在の文法カバレッジの詳細は [`grammar_coverage.md`](grammar_coverage.md) を参照してください。

## 依存クレート

| クレート | 用途 |
|---|---|
| [rowan](https://crates.io/crates/rowan) | ロスレス構文木（CST） |
| [tower-lsp](https://crates.io/crates/tower-lsp) | LSP プロトコル実装 |
| [tokio](https://crates.io/crates/tokio) | 非同期ランタイム |
| [phf](https://crates.io/crates/phf) | コンパイル時完全ハッシュマップ（辞書） |

## ライセンス

リファレンス文法 (src-js/ilmentufa) は [MIT License](src-js/ilmentufa/LICENSE) の下で提供されています。
