# **Rule**: multiple fileds validation

syn の v2 への移行に伴い、rule も下記のようなインターフェースに統一できないかを検討していました。

```rust,ignore
use serde_json::json;
use serde_valid::Validate;

fn sample_rule(_val1: &i32, _val2: &str) -> Result<(), serde_valid::validation::Error> {
    Ok(())
}

#[derive(Validate)]
#[rule(|s| sample_rule(s.val2, s.val1))]
struct SampleStruct {
    val1: String,
    val2: i32,
}

let s = SampleStruct {
    val1: "val1".to_owned(),
    val2: 1,
};

assert!(s.validate().is_ok());
```

このコードは２つの点の問題を抱えています。

### ルール関数の引数が参照ではなくなる。

```rust,ignore
fn sample_rule(_val1: i32, _val2: String) -> Result<(), serde_valid::validation::Error> {
    Ok(())
}
```

これは、実際にはより素晴らしい解決策かもしれません。
常にクロージャに渡すことで、所有権の重複を回避することができるようです。
（参照渡しをしていたのは、複数のカスタムバリデーションを用いる際にコピーを持たないフィールドでは所有権の問題に遭遇する為です）。

この思わぬ効果は、より自然なカスタムバリデーションを書くことができるようになります。
しかし、全てをクロージャに包む場合、関数が都度生成されるため、バイナリサイズやパフォーマンスに影響が出る可能性があります。

### クロージャの引数の型を指定しないといけない。
実際には、こう書かねばコンパイルに失敗します。

```rust,ignore
#[derive(Validate)]
#[rule(|s: &Self| sample_rule(&s.val2, &s.val1))]
struct SampleStruct {
    val1: String,
    val2: i32,
}
```

これはかなり冗長です。

### 将来的には custom との統一

構造体を引数としたクロージャを書けることにより、 rule は custom と同じインターフェースになります。


```rust,ignore
fn sample_rule(val1: &SampleStruct) -> Result<(), serde_valid::validation::Error> {
    Ok(())
}

#[derive(Validate)]
#[rule(sample_rule)]
struct SampleStruct {
    val1: String,
    val2: i32,
}
```

その場合、 rule を用意する理由はありません。

上の問題を上手く解決できた場合、 rule は非推奨になるでしょう。

一旦、クロージャへの対応に関して、特別なロジックを追加しました。
こちらの方が既存の実装を利用して作成しやすかった為です。
