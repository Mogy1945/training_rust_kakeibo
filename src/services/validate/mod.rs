// `InputValidator`という公開構造体を定義します。
// この構造体は、入力値のバリデーション（検証）を行うために使用されます。
pub struct InputValidator {}

// `InputValidator`構造体に対する実装を定義します。
impl InputValidator {
    // `validate_service_type`という公開メソッドを定義します。
    // このメソッドは、サービスの種類を表す`service_type`引数を受け取り、
    // その値が有効な範囲内（0または1）にあるかを検証します。
    pub fn validate_service_type(service_type: u8){
        // `match`式を使用して`service_type`の値を検証します。
        match service_type {
            // `service_type`が0または1の場合、何もしません（{}で表される空のブロック）。
            0 | 1 => {},
            // それ以外の値の場合、エラーメッセージを表示してプログラムを強制終了します。
            // `panic!`マクロは、プログラムの実行を停止させ、指定されたメッセージと共にエラーを報告します。
            _ => panic!("入力値が不正です。")
        }
    }

    pub fn validate_register_type(register_type: u8){
        match register_type {
            0 | 1 => {},
            _ => panic!("入力値が不正です。")
        }
    }

    pub fn validate_category_type(register_type: u8, category_type: u8){
        if register_type == 0 {
            match category_type {
                0 | 1 | 2 => {},
                _ => panic!("カテゴリの入力値が不正です。")
            }
        } else {
            match category_type {
                0 | 1 | 2 => {},
                _ => panic!("カテゴリの入力値が不正です。")
            }
        }
    }
}
