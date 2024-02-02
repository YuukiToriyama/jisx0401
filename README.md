## jisx0401

[![Docs](https://docs.rs/jisx0401/badge.svg)](https://docs.rs/jisx0401)
[![Crates.io (latest)](https://img.shields.io/crates/v/jisx0401)](https://crates.io/crates/jisx0401)

JIS X 0401により定義された都道府県コードを取り扱うためのユーティリティです

### Example
```rust
use jisx0401::Prefecture;

fn main() {
    let prefecture = Prefecture::try_from("26").unwrap();
    assert_eq!(prefecture, Prefecture::KYOTO);
    assert_eq!(prefecture.name_ja(), "京都府");

    let prefecture = Prefecture::try_from("滋賀県").unwrap();
    assert_eq!(prefecture, Prefecture::SHIGA);
    assert_eq!(prefecture.code(), "25");
}
```
