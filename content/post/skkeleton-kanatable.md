---
title: "Skkeleton KanaTable"
date: 2023-01-01T21:27:28+09:00
draft: false
---

# これは何

自分のskkeletonのkanatableを解説するよ

詳しくは[ここらへん](https://github.com/qitoy/dotfiles/blob/main/.vim/autoload/vimrc.vim#L42)を見ればいいんじゃない？(おい)

<!--more-->

# おことわり

- これ書きながら追加していってるから仕様が異なる可能性があります
- プラグイン化はしないの？←後述するように環境依存が激しいので予定はないです

# 前提知識

- skkeletonについて
  - [skkeleton](https://github.com/vim-skk/skkeleton)とは、[denops](https://github.com/vim-denops/denops.vim)製の日本語入力プラグインである。
  - `skkeleton#register_kanatable()`を用いてkanatableに登録をする。
- 自分の環境について
  - キー配列はJIS配列上でProgrammer Dvorakを使っている。

# 解説

## かな配列

[gACT10](http://hp.vector.co.jp/authors/VA002116/gact/gact10_doc.html)をベースに、Programmer Dvorak用に書き換えて、いくつかの拡張を削除した。記事執筆時点では、

- 基本的な打ち方
- 拗音の打ち方(基本)
- 拗音の打ち方(外来語)
- 頻出拗音の省略打ち
- 拗音+ク・ツの省略打ち
- その他の頻出文字列の省略打ち
  - やって良かった打ち
- 記号類

が実装されている。もし必要があれば極楽打ち(く音省略)なども追加するつもりだ。

## 制御キー

SKKでは変換のためにshiftを利用するが、これがDvorak配列と相性が悪いためにsticky-keyを利用している。現在では

```vim
{ ":": "abbrev",
\ ";": "henkanPoint",
\ "@": "katakana",
\ "\<Space>": "henkanFirst",
\}
```

が登録されている。(SKK状態の切り替えは`<C-j>`で行うよう設定している。)

# おわりに

skkeletonのテストも兼ねて軽い記事を書いてみた。もしプラグイン化するなら普通のDvorakやQwertyとの切り替え、デフォルトでの記号類・制御キーの有無などが求められるか。
