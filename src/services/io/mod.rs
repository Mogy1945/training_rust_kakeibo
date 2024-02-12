// `crate::models`モジュールを使用可能にします。これにより、`models`モジュール内の定義された構造体や関数を利用できます。
use crate::models;
// 標準ライブラリからファイル操作に必要なモジュールをインポートします。
use std::fs::File;
use std::io::BufReader;
use std::io::{self, Write};

// `read_data_or_create_new_data`関数を公開します。この関数はファイルパスを引数に取り、`models::Item`のベクターを返します。
pub fn read_data_or_create_new_data(file_path: &str) -> Vec<models::Item>{
    // 指定されたファイルパスでファイルを開きます。`File::open`は`Result`型を返すため、操作の成否に応じて`Ok`または`Err`が返されます。
    let file = File::open(file_path);
    // `match`文を使用して、ファイルのオープン操作の結果に基づいて異なるアクションを行います。
    match file {
        // ファイルのオープンに成功した場合、ファイルハンドル`f`を取得します。
        Ok(f) => {
            // ファイルハンドルからバッファリーダーを作成します。これにより、ファイルの読み込み効率が向上します。
            let buf_reader = BufReader::new(f);
            // `serde_json::from_reader`を使用して、バッファリーダーからJSONを読み込み、`models::Item`のベクターにデシリアライズします。
            // デシリアライズに失敗した場合は、プログラムをパニックさせて終了します。
            serde_json::from_reader(buf_reader).expect("デシリアライズに失敗しました")
        },
        // ファイルのオープンに失敗した場合（例えば、ファイルが存在しない場合など）は、新規ファイルを作成することを示すメッセージを表示し、
        // 空の`Vec<models::Item>`を返します。
        Err(_) => {
            println!("新規ファイルを作成します");
            Vec::new()
        }
    }
}

// `read_data_or_panic`関数を定義します。この関数はファイルパスを引数に取り、`models::Item`のベクターを返します。
// この関数は、ファイルが開けない場合やデシリアライズに失敗した場合にプログラムをパニックさせます。
pub fn read_data_or_panic(file_path: &str) -> Vec<models::Item>{
    // `File::open`を使用してファイルを開きます。ファイルが開けない場合は、`expect`メソッドによりエラーメッセージと共にプログラムをパニックさせます。
    let file = File::open(file_path).expect("ファイルがオープンできませんでした");
    // ファイルハンドルからバッファリーダーを作成します。これにより、ファイルの読み込み効率が向上します。
    let buf_reader = BufReader::new(file);
    // `serde_json::from_reader`を使用して、バッファリーダーからJSONを読み込み、`models::Item`のベクターにデシリアライズします。
    // デシリアライズに失敗した場合は、`expect`メソッドによりエラーメッセージと共にプログラムをパニックさせます。
    let data: Vec<_> = serde_json::from_reader(buf_reader).expect("デシリアライズに失敗しました");

    // 読み込んだデータが空の場合、プログラムをパニックさせます。
    if data.len() == 0 {
        panic!("データが存在しません");
    }
    // デシリアライズされたデータを返します。
    data
}

// `write_to_json`関数を定義します。この関数は、`models::Item`のベクターとファイルパスを引数に取ります。
pub fn write_to_json(data: &Vec<models::Item>, file_path: &str) -> io::Result<()> {
    // `serde_json::to_string_pretty`を使用して、引数で受け取ったデータを整形されたJSON文字列にシリアライズします。
    // シリアライズに失敗した場合は、`expect`メソッドによりエラーメッセージと共にプログラムをパニックさせます。
    let json_data = serde_json::to_string_pretty(data).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    // `File::create`を使用して指定されたファイルパスに新しいファイルを作成します。ファイルの作成に失敗した場合は、
    // `expect`メソッドによりエラーメッセージと共にプログラムをパニックさせます。
    println!("ファイルパス: {}", file_path);
    let mut file = File::create(file_path)?;
    println!("ファイル: {:?}", file);

    // `writeln!`マクロを使用して、シリアライズされたJSONデータをファイルに書き込みます。
    // 書き込みに失敗した場合は、`expect`メソッドによりエラーメッセージと共にプログラムをパニックさせます。
    writeln!(file, "{}", json_data)?;
    // データの書き込みが完了したことをユーザーに通知します。
    println!("項目の登録が完了しました！");
    Ok(())
}
