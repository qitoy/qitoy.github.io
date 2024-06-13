---
title: "SATySFiでレポート課題を書いた"
date: 2023-10-25T10:39:23+09:00
tags:
- SATySFi
draft: false
summary: "SATySFiの書き方とか"
---

# 目標

実際の課題を載せるのはよくないので下のPDFのものを最終目標とする。

{{< pdf src="./hoge.pdf" width="100%" height="1000px" >}}

# 使用パッケージ一覧

- [azmath](https://satyrographos-packages.netlify.app/packages/azmath)
    - 数式を書きやすくする
- [class-exdesign](https://satyrographos-packages.netlify.app/packages/class-exdesign)
    - `+section`の表示をいじるため
- [code-printer](https://satyrographos-packages.netlify.app/packages/code-printer)
    - ソースコードを貼れる
- [bibyfi](https://satyrographos-packages.netlify.app/packages/bibyfi)
    - 参考文献を貼れる
- [figbox](https://satyrographos-packages.netlify.app/packages/figbox)
    - 図表を上手く貼れる

# 書く

## 注意

SATySFiのsyntax highlightが無いのでみづらいかも

## sectionをいじる

ます`+section`の表示をnから問nにしたい。このとき
```
(| <record> with <label> = <new-value> |)
```
という構文が使える。これは`<record>`の`<label>`を`<new-value>`にしたものを返す。よって、`document`を、
```satysfi
document(|
  title = { レポート };
  author = { qitoy しゅどぼ };
  date = {};
  show-title = true;
  show-toc = false;
  style = ArticleJa.a4paper;
  design = (|
    ArticleJa.article with section-num-function = fun _ i -> `問` ^ arabic i;
  |);
  header-footer = ArticleJa.normalHF;
  fonts = ArticleJa.fonts;
|) '<>
```
などとすればよい。

## ソースコード、実行結果を貼る

これは単純に`+code-printer`や`+file-printer`をすればよいが、それぞれで書式を変えたくなる。なので、`let in`でそれぞれの変数に束縛させればよい。例

```satysfi
let source = 
  CodePrinter.default
    |> CodePrinter.set-syntax CodeSyntax.rust
    |> CodePrinter.set-theme CodeTheme.basic-light
    |> CodePrinter.set-line-break-mark (fun _ _ -> (inline-nil, inline-nil)) % 行のwrapにマーカーを付けない

let prompt =
  CodePrinter.default
    |> CodePrinter.set-line-break-mark (fun _ _ -> (inline-nil, inline-nil))
    |> CodePrinter.set-number-fun CodeDesign.number-fun-null % 行番号を付けない
in
```

## 参考文献を貼る

今回はwebサイトを貼りたいが、urlと著者を書かなくてはならない。urlは冗長なので(?)、hyper linkで埋め込むことにする。bibyfiにはちょうどよい書式が無さそうなので自前で用意しなくてはならない。

`bibyfi-item`には`Manual`を選んだ。ここで欲しい要素は、`title`、`address`、`author`のみなので、すべてデフォ値のレコードを用意してさっきの構文で書き換えるほうが良い。

```satysfi
let bib-default = 
  (|
    title = ` `;
    author = None;
    organization = None;
    address = None;
    edition = None;
    month = None;
    year = None;
    note = None;
    key = None;
  |)

let bibs =
  [
    (`aho`, Manual(|
      bib-default with title = `桂三度`;
      author = Some([`Wikipedia`;]);
      address = Some(`https://ja.wikipedia.org/wiki/%E6%A1%82%E4%B8%89%E5%BA%A6`);
    |));
  ]
```

そしてこのレコードをmarkdownでいう`[title, author](address)`に変換する`bibyfi-theme`を書けばよい。

```satysfi
let mk-index ctx index =
  let s = `[` ^ arabic index ^ `] `# in read-inline ctx (embed-string s)

let mk-manual r =
  match (r#author, r#address) with
  | (Some([author]), Some(address)) ->
    let inner = embed-string (r#title ^ `, `# ^ author) in { \href (address) (inner); }
  | _ -> { invalid }

let bibyfi-theme ctx index bib-item =
  match bib-item with
    | Manual(r) -> BiByFi.make-entry ctx (mk-index ctx index) (read-inline ctx (mk-manual r))
    | _ -> BiByFi.make-entry ctx (mk-index ctx index) (read-inline ctx { invalid })
```

## 数式を書く

azmathのドキュメントを読めばよい。特記すべき事項は、新しい括弧を定義したいときや、複数文字の変数などを定義したいとき、以下のようにする。

```satysfi
let-math \range m1 m2 = math-paren AZMathParens.square-bracket-l AZMathParens.round-bracket-r ${#m1 , #m2}
let-math \len = math-char MathOrd `len`
```

## 図を書く

通常、`graphics`に文字を埋め込むのは難しそう（ちゃんと調べてない）なのだが、`FigBox.graffiti-given-context`を使うと`context`を扱いやすくなる。以下のような関数を定義すると部分適応したものを渡すだけでよくなり、お得(?)。

```satysfi
func : args -> context -> graphics list
```

# おわりに

ここまでする必要無かったなあ()
