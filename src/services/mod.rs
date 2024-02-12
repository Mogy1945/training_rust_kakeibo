// `services`モジュール内で`validate`モジュールを公開します。
// この行により、`services`モジュールの外部から`validate`モジュールにアクセスできるようになります。
// `validate`モジュールは、入力値のバリデーション（検証）に関連する機能を提供します。
pub mod validate;
pub mod io;
pub mod register;
