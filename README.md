
# FF14 Avatar Fetcher

このRustライブラリは、ファイナルファンタジーXIV（FF14）のキャラクターのアバターを取得するために設計されています。キャラクターIDを使用してキャラクターアバターを取得し、キャラクターのID、名前、およびアバター画像のURLを返します。

## 特徴

- 単一または複数のFF14キャラクターアバターを取得。
- 効率的なネットワークリクエストのための非同期API。
- Rustアプリケーションと簡単に統合できます。

## インストール

`Cargo.toml`ファイルに`ff14_avatar`を追加してください：

```toml
[dependencies]
ff14_avatar = "0.1.0"
```

## 使用方法

このライブラリを使用するには、まずプロジェクトに含める必要があります：

```rust
use ff14_avatar::{fetch_avatar, fetch_avatars};
```

### 単一のアバターを取得

単一のFF14キャラクターアバターを取得するには、`fetch_avatar`関数を使用します。以下はその例です：

```rust
use ff14_avatar::fetch_avatar;

#[tokio::main]
async fn main() {
    let id = "あなたのキャラクターID";
    let avatar = fetch_avatar(id).await;
    println!("ID: {}", avatar.id);
    println!("Name: {}", avatar.name);
    println!("ImageUrl: {}", avatar.image_url);
}
```

### 複数のアバターを取得

複数のアバターを取得するには、キャラクターIDのベクターを使用して`fetch_avatars`関数を使用します：

```rust
use ff14_avatar::fetch_avatars;

#[tokio::main]
async fn main() {
    let ids = vec!["キャラクターID_1".to_string(), "キャラクターID_2".to_string()];
    let avatars = fetch_avatars(ids).await;
    for avatar in avatars {
        println!("ID: {}", avatar.id);
        println!("Name: {}", avatar.name);
        println!("ImageUrl: {}", avatar.image_url);
    }
}
```

## 依存関係

- `reqwest` HTTPリクエストを行うため。
- `scraper` HTMLコンテンツを解析するため。
- `tokio` 非同期ランタイムのため。

## 貢献

貢献を歓迎します！プルリクエストを送信したり、バグを報告したり、機能を提案したり、自由に行ってください。

## ライセンス

このプロジェクトはMITライセンスの下でライセンスされています - 詳細はLICENSEファイルを参照してください。
