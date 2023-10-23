---
title: "もっとブログ書け"
date: 2023-10-22T16:07:56+09:00
draft: false
---

# これは何

Hugoでのブログの書き方を忘れてしまった……（随時更新）

<!--more-->


# 書き方

まずプロジェクトルート（submoduleが必須）で
```bash
hugo new post/hoge.md
# or: hugo new post/hoge/index.md
```
をする。下の方が画像とか置きやすくてよい

`title`は自由にいじる、公開するには`draft`を`false`にするのを忘れずに

```bash
hugo server
```
をすると書きながらブラウザの`localhost:1313`で確認できる。ここでエラーが出て表示できないときはsubmoduleのロードを忘れているかもなのでやる

できたらpushしましょう

## 画像を埋め込む

まず`index.md`の方法で記事を作成して、そのディレクトリに画像を置く。
hugo特有の形式（ショートコードShortcodesという）として、
```md
{{</* figure src="hoge.png" alt="手書きのhoge" width="320" height="320" */>}}
```
とするといいかんじになる。↓例
{{< figure src="hoge.png" alt="手書きのhoge" width="320" height="320" >}}

ちなみにコードブロックでの例示はエスケープが必要で、
```
{{</*/* aaa */*/>}}
```
とする。

# おわりに

なんか改造とかしたいなあ
