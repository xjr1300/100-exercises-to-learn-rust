//! チケット管理システムREST APIサーバー
//!
//! ```sh
//! # ヘルスチェック
//! $ curl http://localhost:3000
//! Hello, World!
//!
//! # チケットを取得（エラー）
//! $ curl --include http://localhost:3000/tickets/0
//! HTTP/1.1 404 Not Found
//! content-type: application/json
//! content-length: 33
//! date: Tue, 16 Jul 2024 01:51:03 GMT
//!
//! {"error":"Ticket does not exist"}
//!
//! # 1つ目のチケットを登録
//! $ curl --include -H "Content-Type: application/json" -d '{"title": "吾輩は猫である", "description": "猫の目を通じて人間社会を風刺した作品"}' http://localhost:3000/tickets
//! HTTP/1.1 200 OK
//! content-type: application/json
//! content-length: 8
//! date: Tue, 16 Jul 2024 02:22:31 GMT
//!
//! {"id":0}
//!
//! # 2つ目のチケットを登録
//! $ curl -H "Content-Type: application/json" -d '{"title": "羅生門", "description": "人間が生きるための利己主義と善悪について描いた作品"}' http://localhost:3000/tickets
//! {"id":1}
//!
//! # 1つ目のチケットを取得
//! $ curl --include http://localhost:3000/tickets/0
//! HTTP/1.1 200 OK
//! content-type: application/json
//! content-length: 139
//! date: Tue, 16 Jul 2024 02:03:38 GMT
//!
//! {"id":0,"title":"吾輩は猫である","description":"猫の目を通じて人間社会を風刺した作品","status":"ToDo","version":0}
//!
//! # 2つ目のチケットを取得
//! $ curl http://localhost:3000/tickets/1
//! {"id":1,"title":"羅生門","description":"人間が生きるための利己主義と善悪について描いた作品","status":"ToDo","version":0}
//!
//! # 2つ目のチケットの状態を`InProgress`に更新
//! $ curl --include -X PATCH -H "Content-Type: application/json" -d '{"status": "InProgress", "version": 0}' http://localhost:3000/tickets/1
//! HTTP/1.1 200 OK
//! content-length: 0
//! date: Tue, 16 Jul 2024 02:12:03 GMT
//!
//! # 2つ目のチケットを取得
//! $ curl http://localhost:3000/tickets/1
//! {"id":1,"title":"羅生門","description":"人間が生きるための利己主義と善悪について描いた作品","status":"InProgress","version":1}%
//!
//! # 誤ったバージン番号で2つ目のチケットの状態を`Done`に更新（エラー）
//! $ curl --include -X PATCH -H "Content-Type: application/json" -d '{"status": "Done", "version": 0}' http://localhost:3000/tickets/1
//! HTTP/1.1 400 Bad Request
//! content-type: application/json
//! content-length: 55
//! date: Tue, 16 Jul 2024 02:15:33 GMT
//!
//! {"error":"The version of updating ticket is not match"}
//! ```
use ticket_store::server;

#[tokio::main]
async fn main() {
    server::run().await;
}
