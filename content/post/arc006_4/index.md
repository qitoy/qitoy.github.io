---
title: "アルファベット探し"
date: 2023-03-03T21:55:08+09:00
tags:
- AtCoder
draft: false
summary: "自分の解法のメモ"
---

# 問題

https://atcoder.jp/contests/arc006/tasks/arc006_4

# 解法

A、B、Cの黒マスの数を数えると、(A, B, C) = (12, 16, 11)である。また、これをk倍に拡大するとそれぞれk^2倍になる。このとき、Aのk_1倍とBのk_2倍では黒マスの個数が一致しない。他同様。したがって、上下左右斜めに隣接している黒マスの個数をUnionFindなどで数え、それらを分類すればよい。

# 提出

Rust
https://atcoder.jp/contests/arc006/submissions/39377821

# おわりに

古いコンテストのメモリ制限厳しいのなんとかなりませんか(1敗)
