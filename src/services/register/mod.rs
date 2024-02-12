// 標準ライブラリから必要なモジュールをインポートします。
use std::{io, str::FromStr};
// chronoクレートからNaiveDate型をインポートします。これは日付を扱うための型です。
use chrono::NaiveDate;
// 同じクレート内の他のモジュールをインポートします。
use crate::services;
use crate::models;

// `run`関数は、ユーザーからの入力を受け取り、収支の登録を行うメインの関数です。
pub fn run(file_path: &str) {
    // 収支の登録開始をユーザーに通知します。
    println!("収支の登録を行います");
    // 登録種別（収入か支出か）をユーザーから入力させます。
    let register_type = input_register_type();
    // 品目名をユーザーから入力させます。
    let name = input_name();
    // カテゴリ種別（給与、ボーナスなど）をユーザーから入力させます。
    let category_type = input_category_type(register_type);
    // 金額をユーザーから入力させます。
    let price = input_price();
    // 日付をユーザーから入力させます。
    let date = input_date();
    // 入力された登録種別とカテゴリ種別から、カテゴリの詳細を取得します。
    let category = models::Item::get_category(register_type, category_type);

    // 入力された情報から新しい項目を作成します。
    let item = models::Item::new(name, category, price, date);

    // 指定されたファイルパスから既存のデータを読み込み、新しいデータがない場合は新しいデータを作成します。
    let mut data = services::io::read_data_or_create_new_data(file_path);
    // 新しく作成した項目をデータに追加します。
    data.push(item);

    // 更新されたデータをJSON形式でファイルに書き込みます。書き込みに失敗した場合はエラーメッセージを表示し、プログラムを終了します。
    services::io::write_to_json(&data, file_path).unwrap_or_else(|e| {
        eprintln!("データのJSONへの書き込み中にエラーが発生しました: {}", e);
        std::process::exit(1);
    });
}

// 登録種別をユーザーから入力させる関数です。
fn input_register_type() -> u8 {
    // ユーザーに登録種別の入力を促します。
    println!("登録種別を入力してください（0:収入、1:支出）");
    // 入力を格納するための新しいStringオブジェクトを作成します。
    let mut input = String::new();
    // 標準入力からの読み込みを試み、エラーがあればプログラムを終了します。
    io::stdin().read_line(&mut input).expect("登録種別の入力に失敗しました");
    // 入力をu8型に変換し、失敗した場合はプログラムを終了します。
    let register_type: u8 = input.trim().parse().expect("登録種別は数値を入力してください");
    // 入力された登録種別が有効かどうかを検証します。
    services::validate::InputValidator::validate_register_type(register_type);
    // 登録種別を返します。
    register_type
}

// 品目名をユーザーから入力させる関数です。
fn input_name() -> String {
    // ユーザーに品目名の入力を促します。
    println!("品目名を入力してください");
    // 入力を格納するための新しいStringオブジェクトを作成します。
    let mut name = String::new();
    // 標準入力からの読み込みを試み、エラーがあればプログラムを終了します。
    io::stdin().read_line(&mut name).expect("品目名の入力に失敗しました");
    // 入力された品目名の前後の空白を削除し、Stringとして返します。
    name.trim().to_string()
}

// カテゴリ種別をユーザーから入力させる関数です。
fn input_category_type(register_type: u8) -> u8 {
    // ユーザーにカテゴリの入力を促します。
    println!("カテゴリを入力してください");
    // 登録種別に応じて、カテゴリの選択肢を表示します。
    if register_type == 0 {
        println!("0:給与、1:ボーナス、2:その他");
    } else {
        println!("0:食費、1:趣味、2:その他");
    }
    // 入力を格納するための新しいStringオブジェクトを作成します。
    let mut category_type = String::new();
    // 標準入力からの読み込みを試み、エラーがあればプログラムを終了します。
    io::stdin().read_line(&mut category_type).expect("カテゴリ種別の入力に失敗しました");
    // 入力をu8型に変換し、失敗した場合はプログラムを終了します。
    let category_type: u8 = category_type.trim().parse().expect("カテゴリは数値を入力してください");
        // 入力されたカテゴリ種別が有効かどうかを検証します。
        services::validate::InputValidator::validate_category_type(register_type, category_type);
        // カテゴリ種別を返します。
        category_type
    }
    
    // 金額をユーザーから入力させる関数です。
    fn input_price() -> u32 {
        // ユーザーに金額の入力を促します。
        println!("金額を入力してください");
        // 入力を格納するための新しいStringオブジェクトを作成します。
        let mut price = String::new();
        // 標準入力からの読み込みを試み、エラーがあればプログラムを終了します。
        io::stdin().read_line(&mut price).expect("金額の入力に失敗しました");
        // 入力をu32型に変換し、失敗した場合はプログラムを終了します。
        let price: u32 = price.trim().parse().expect("金額は数値を入力してください");
        // 金額を返します。
        price
    }
    
    // 日付をユーザーから入力させる関数です。
    fn input_date() -> NaiveDate {
        // ユーザーに日付の入力を促します（yyyy-mm-dd形式）。
        println!("日付を入力してください（yyyy-mm-dd）");
        // 入力を格納するための新しいStringオブジェクトを作成します。
        let mut date = String::new();
        // 標準入力からの読み込みを試み、エラーがあればプログラムを終了します。
        io::stdin().read_line(&mut date).unwrap();
        // 入力をNaiveDate型に変換し、失敗した場合はプログラムを終了します。
        NaiveDate::from_str(&date.trim()).expect("日付はyyyy-mm-ddの形式で入力してください")
    }
