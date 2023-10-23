---
title: "しゅどぼのPC環境"
date: 2023-10-23T21:02:53+09:00
draft: false
---

# これは何

Asahi Linuxのはなし

<!--more-->

# おおまかな環境

MacとArch Linux(Asahi Linux)のデュアルブートですが、Mac側は今はあんまり使ってないのでLinux側の話をします。

# Linux環境

## neofetch

{{< figure src="neofetch.png" alt="neofetch" >}}

## PCについて

見てのとおりMacBook Air M1のメモリ8GBです。ギリ人権

## デスクトップ環境・ウィンドウマネージャなど

swayを採用しています。DEがsway-sessionとなっているのは、systemd userを使用するやつです[参考](https://github.com/swaywm/sway/wiki/Systemd-integration#running-sway-itself-as-a---user-service)。主な設定は[ここ](https://github.com/qitoy/dotfiles/blob/main/config/sway/config)にありますが、ほとんどがi3のデフォのものから引っ張ってきただけなので、ちゃんとは使えてはいません（おい）。

壁紙は最初は無かったのですが、[配布](https://twitter.com/OverRapid/status/1386949369581109252)されているものを現在使用しています。

## ターミナル・エディタなど

WezTermでシェルはzsh、エディタはNeovimです。最初はVimを使用していましたが、tree-sitterを使いたく、Neovimに移りました。Neovimの設定は[ここから](https://github.com/qitoy/dotfiles/tree/main/vim)。

## IMEについて

IMEはインストールしておらず、代わりに[skkeleton](https://github.com/vim-skk/skkeleton)で日本語入力をし、それをコピペする運用をしています。キー配列がdvp(programmer dvorak)で、日本語の配列がgACT10をベースとしているので、なかなかにカオスです。

# おわりに

なにかあったら追加します。Neovimについては別で記事を作りたい
