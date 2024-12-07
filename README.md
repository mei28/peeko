```
peeko/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── app.rs
│   ├── tasks/
│   │   ├── mod.rs
│   │   ├── task.rs
│   │   ├── parser.rs        // ファイルタイプ判定・パーサ振り分け
│   │   ├── parsers/
│   │   │   ├── mod.rs
│   │   │   ├── makefile_parser.rs
│   │   │   ├── package_json_parser.rs
│   │   │   └── pyproject_toml_parser.rs
│   │   └── detector.rs      // 拡張子やファイル名からどのパーサを使うか判断
│   ├── preview/
│   │   ├── mod.rs
│   │   ├── previewer.rs     // 指定行周辺の抜粋取得
│   │   └── highlighter.rs   // シンタックスハイライト機能
│   ├── ui/
│   │   ├── mod.rs
│   │   ├── render.rs
│   │   └── layout.rs        // レイアウト関連
│   └── utils.rs
└── tests/
    ├── cli_test.rs
    ├── integration_test.rs
    ├── tasks_test.rs
    ├── fixtures/
    │   ├── Makefile
    │   ├── package.json
    │   └── pyproject.toml
    └── ...
```
