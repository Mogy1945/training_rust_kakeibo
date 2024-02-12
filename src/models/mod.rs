// serdeライブラリからSerialize, Deserializeトレイトをインポートします。これにより、構造体や列挙型をシリアライズ（データを連続的な形式に変換）およびデシリアライズ（データを元の形式に戻す）できます。
use serde::{Serialize, Deserialize}; 
// chronoライブラリからNaiveDate型をインポートします。これは、タイムゾーンを考慮しない日付を扱うための型です。
use chrono::{NaiveDate,Datelike}; 

// NOTE: #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
//       特定のトレイトを自動的に実装するようコンパイラに指示します。
//       この行が付与された構造体や列挙型に対して、以上のトレイトが自動的に実装されます。
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
// 収入のカテゴリを表す列挙型です。
pub enum IncomeCategory{
    Salary, // 給料
    Bonus, // ボーナス
    Other, // その他の収入
}

// 支出のカテゴリを表す列挙型です。
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum ExpenseCategory{
    Food, // 食費
    Hobby, // 趣味に関する支出
    Other, // その他の支出
}

// 収入または支出のカテゴリを表す列挙型です。
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Category{
    Income(IncomeCategory), // 収入カテゴリ
    Expense(ExpenseCategory), // 支出カテゴリ
}

// 財務アイテムを表す構造体です。
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Item {
    name: String, // アイテムの名前
    category: Category, // アイテムのカテゴリ（収入または支出）
    price: u32, // アイテムの価格
    date: NaiveDate, // アイテムの日付
}

// Item構造体に関する実装です。
impl Item {
    // 新しいItemインスタンスを生成するための関数です。
    // 引数:
    //   name: String - アイテムの名前
    //   category: Category - アイテムのカテゴリ（収入または支出）
    //   price: u32 - アイテムの価格
    //   date: NaiveDate - アイテムの日付
    // 戻り値: Self - 新しいItemインスタンス
    pub fn new(name:String, category:Category, price:u32, date:NaiveDate) -> Self {
        Item {name, category, price, date}
    }

    pub fn get_category(register_type: u8,category_type: u8) -> Category {
        if register_type == 0 {
            match category_type {
                0 => Category::Income(IncomeCategory::Salary),
                1 => Category::Income(IncomeCategory::Bonus),
                2 => Category::Income(IncomeCategory::Other),
                _ => panic!("不正なカテゴリ種別です"),
            }
        } else {
            match category_type {
                0 => Category::Expense(ExpenseCategory::Food),
                1 => Category::Expense(ExpenseCategory::Hobby),
                2 => Category::Expense(ExpenseCategory::Other),
                _ => panic!("不正なカテゴリ種別です"),
            }
        }
    }

    pub fn get_year(&self) -> i32 {
        self.date.year()
    }

    pub fn get_month(&self) -> u32 {
        self.date.month()
    }

    pub fn get_day(&self) -> NaiveDate {
        NaiveDate::from_ymd_opt(self.get_year(), self.get_month(), 1).unwrap()
    }

    pub fn get_price_for_summary(&self) -> i32 {
        match self.category {
            Category::Income(_) => self.price as i32,
            Category::Expense(_) => -1 * self.price as i32,
        }
    }
}
