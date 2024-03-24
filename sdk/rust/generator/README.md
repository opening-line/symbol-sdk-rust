# python catbuffer generator

## Generate catbuffer

```sh
./scripts/run_catbuffer_generator.sh
```

# sdkの達成目標
`symbol/tree/rust/tests/vectors`にあるテストベクタをパスすることが達成目標になります。
nemとsymbolがありますが、とりあえずはsymbolのみに対応すると良いそうです。

使いやすいsdkである必要があるため、綺麗に解く必要がありますが、
入力と出力(期待値)に対してパスするコードを書くという点で凡そAtcoderと同じだと考えています。
RustやC++での解法(sdk)はまだ存在しませんが、jsやpythonの解法(sdk)は存在するので、
それらを見ながら解くことができます。
入力と出力が明示されていないので、jsやpythonを見て判断しました。

# 進捗
テストベクタを大別するとcryptoとmodelsがあります。
cryptoのほとんどをとりあえずはパスするようにしました。
最初は7.test-voting-keys-generation.jsonに取り組んで貰うのが良いと考えています。

## crypto
- [x] 0.test-keccak-256.json
- [x] 0.test-sha3-256.json
- [x] 1.test-address.json
- [x] 1.test-keys.json
- [x] 2.test-sign.json
- [x] 3.test-derive-hkdf.json
- [x] 4.test-cipher.json
- [x] 5.test-mosaic-id.json
- [x] 6.test-hd-derivation.json
- [x] 7.test-voting-keys-generation.json

## models
- [ ] blocks.json
- [ ] other.json
- [ ] receipts.json
- [ ] transactions.json

# rust sdkの全体概要
以下に全体概要を示します。
sdkを使うユーザはprelude.rsしかuse(多言語で言うとincludeやimport)する必要がないように設計しています。
sdkの使用方法についてはsymbol_test_vectors.rsを参考にしてください。
examplesは作成できていません。

``` mermaid
    graph LR;
    n0(models_header.rs);
    n1(models.rs);
    n2(address.rs)
    n3(cipher.rs)
    n4(key.rs)
    n5(bip.rs)
    n6(prelude.rs)
    n7(symbol_test_vectors.rs)
    n8((rust sdkの外のsymbol/catbuffer))

    n8 --"generator/Generator.py を用いて生成"--> n1;

    n0 --> n1;
    n1 --> n2;
    n1 --> n3;
    n1 --> n4;
    n1 --> n5;
    n1 --> n6;

    n2 --> n6
    n3 --> n6
    n4 --> n6
    n5 --> n6

    n6 --> n7;
```

# 改修方法
対象のテストをパスするためにsymbol_test_vectors.rsにコードを記述します。
その過程で以下の工程が必要になります。

1. 外部ライブラリを活用可能かを考えます。
2. 利用可能な外部ライブラリが無い場合、addoress.rsやcipher.rsの位置にファイルを追加、または編集することを試みます。
3. models.rsを編集することを考えます。
しかし、models.rsは自動生成されるものであり、直接編集してもその変更が残りません(一時的に編集することはもちろん大丈夫)。
他言語のsdkと共有しているsymbol/catbufferを編集するのもダメです。
generatorディレクトリのみを編集して対処してください。
4. modelsの大幅改修が必要な場合は、僕に聞いてくれたら嬉しいです。

# 改修テクニック
* コンパイル時間が長くないので、基本的にprintfデバッグで問題ありません。
ただし、無駄な実行を抑制するために適切にコメントアウトを駆使したらデバッグ時間が短くなります。

* テストベクタは多数のケースを含んでおり、printfデバッグしたログを観察する際にスクロールするのがめんどくさいかもしれません。
そういう時は、printfした後にすぐに強制終了すればよいです。以下に例を示します。

``` rust
// rust
dbg!(something);
panic!();
```

``` python
# python
print(something)
exit()
```

``` javascript
// javascript
console.log(something);
exit();
```

* rustにおけるテスト実行時にcargo test --releaseを使用する必要がありますが、
`cargo test --release test6`のように指定すると、任意のテストのみを実行してくれます。

* rustはcargo fmtとすると、コード全体を自動整形してくれます。


# 実践チュートリアル
cryptoテストベクタの内、7.test-voting-keys-generation.jsonが全く手を付けられていません。
既存のrustコードと多言語のsdkを参考にしながら、symbol_test_vectors.rsにコードを追記して、
パスさせてみましょう。

# 環境構築

``` docker
FROM ubuntu:22.04

WORKDIR /root

# 共通
RUN apt update
RUN apt install -y curl gdb build-essential git
RUN git clone https://github.com/YoshizawaShogo/symbol && \
    cd symbol && \
    git switch rust
RUN apt install -y python3 python3-pip
RUN find . -iname *requirements.txt -exec pip3 install -r {} \;

# for Rust
RUN curl "https://sh.rustup.rs" | sh -s -- -y

# for Python
# 特になし

# for JS
RUN apt install -y npm
RUN cd symbol/sdk/javascript && npm install node
```

# テストベクタの検証方法
symbol/tests/vectorsにある

``` bash
# for Rust
cd sdk/rust
cargo test --release
cargo test --release --all-features

# for Python, JS
cd sdk/python # or javascript
./scripts/ci/test_vectors.sh
```

# Models生成方法

``` bash
# 共通
cd sdk/rust # or python, javascript
./scripts/run_catbuffer_generator.sh
```