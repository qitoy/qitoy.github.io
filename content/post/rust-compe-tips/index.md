---
title: "Rustで競プロをするときのTips"
date: 2023-11-03T22:45:23+09:00
tags:
- Rust
- AtCoder
- tips
draft: false
summary: "主にAtCoder"
---

# DFS中に関数から抜け出す

[`?`演算子](https://doc.rust-lang.org/rust-by-example/std/result/question_mark.html)というのがあります。`Option`を使うと上限などを設けることができ、`Result`を使うと1つ見つかったときにそれを返すことができます。

* [`Option`の使用例](https://atcoder.jp/contests/abc284/submissions/47185709)

* [`Result`の使用例](https://atcoder.jp/contests/abc305/submissions/47186933)
