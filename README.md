# rust_studies

Rustの勉強用リポジトリ

## 資料

https://doc.rust-jp.rs/book/second-edition/

## メモ

### rustcによるコンパイル

docker コンテナ内で以下のようにする

```
$ cd projects/ch0102_hello_world/
$ rustc main.rs
$ ./main
```

### cargoによるコンパイル

docker コンテナ内で以下のようにする

```
$ cd projects/ch0103_cargo
$ cargo build
$ ./target/debug/ch0103_cargo
```

その他に以下のコマンドも有用

* cargo run
* cargo check
