// This is our last exercise. Let's go down a more unstructured path!
// Try writing an **asynchronous REST API** to expose the functionality
// of the ticket management system we built throughout the course.
// It should expose endpoints to:
//  - Create a ticket
//  - Retrieve ticket details
//  - Patch a ticket
//
// Use Rust's package registry, crates.io, to find the dependencies you need
// (if any) to build this system.
//
// これは最後の演習です。より構造化されていない道を進みましょう！
// コースを通じて構築したチケット管理システムの機能を公開する**非同期REST API**を記述してください。
// それは、次のエンドポイントを公開します。
// - チケットの作成
// - チケット詳細の取得
// - チケットのパッチ更新
//
// このシステムを構築するために、任意で必要な依存関係を見つけるために、Rustのパッケージレジストリである
// crate.ioを使用してください。

pub mod dto;
pub mod models;
pub mod server;
pub mod store;
