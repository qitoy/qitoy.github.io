---
title: "RE: 素数数えよう"
date: 2024-04-26T22:51:30+09:00
draft: false
math: true
---

# これは何

[素数数えよう](https://qitoy.hatenablog.com/entry/2021/09/27/113420)を見返してみたところ、意味不明であったので、書き直すことにした

<!--more-->

$$
\global\def\paren#1{\left(#1\right)}
\global\def\brace#1{\left\\{#1\right\\}}
\global\def\abs#1{\left|#1\right|}
\global\def\floor#1{\left\lfloor#1\right\rfloor}
\global\def\app#1#2{#1\left(#2\right)}
\global\def\setsep#1#2{\left\\{#1\mid#2\right\\}}
$$

# TL;DR

Meissel-Lehmer algorithmをの簡易的な実装（空間計算量がオリジナルより大きい）をRustで行なった。

[実装](https://github.com/qitoy/rust-library/blob/main/prime/pi/src/lib.rs)

# 用語説明

- \\(\pi(x)\\): \\(x\\)以下の素数の数
- \\(p_a\\): \\(a\ (a \geq 1)\\)番目の素数
    - ただし、\\(p_0\\)は\\(0\\)とする。
- \\(\phi(x, a)\\): \\(x\\)以下の正整数であって素因数が全て\\(p_a\\)より大きいものの個数
- \\(P_k(x, a)\\): \\(x\\)以下の正整数であって、重複込みでちょうど\\(k\\)個の素因数を持ち、その素因数の全てが\\(p_a\\)より大きいものの個数

ここで説明されていないものは適宜説明をする。

# どのようにして計算するか

## 事前準備

まず定義から自明に
$$
\begin{equation*}
\begin{split}
\pi(x) = P_1(x, a) + a \\\\
\phi(x, a) = \sum_{k=0}^\infty P_k(x, a)
\end{split}
\end{equation*}
$$
が成り立つ。
以下、\\(x\\)と\\(a = \pi(x^{1/3})\\)を固定する。すると、\\(P_k(x, a)\ (k \geq 3)\\)はすべて\\(0\\)となる。

{{< details "なぜ？" >}}
\\(p_a\\)は\\(x^{1/3}\\)以下の最大の素数となる。
つまり、\\(n \in P_k(x, a)\\)をとると\\(n\\)の3つ以上ある素因数は\\(x^{1/3}\\)より大きくなくてはならない。
これは\\(n \leq x\\)に反する。
{{< /details >}}

\\(P_0(x, a)\\)は\\(1\\)しか無いので、結局以下の式となり、\\(\phi(x, a)\\)および\\(P_2(x, a)\\)の計算に帰着する。
$$\pi(x) = \phi(x, a) + a - P_2(x, a) - 1$$

## \\(P_2\\)の計算

\\(\app{P_2}{x,a}\\)を定義に沿って式変形する。

$$
\begin{align*}
    \app{P_2}{x, a} &= \abs{\setsep{ n }{ n \leq x, n = p_b p_c \paren{ a < b \leq c } }} \\\\
    &= \sum_{j = a+1}^{\app{\pi}{x^{1/2}}} \abs{\setsep{ n }{ n \leq x, n = p_j p_k \paren{j \leq k \leq \app{\pi}{\frac{x}{p_j}} } }} \\\\
    &= \sum_{j = a+1}^{\app{\pi}{x^{1/2}}} \paren{\app{\pi}{\frac{x}{p_j}} - j + 1} =
    \binom{a}{2} - \binom{\app{\pi}{x^{1/2}}}{2} + \sum_{j=a+1}^{\app\pi{x^{1/2}}}\app\pi{\frac{x}{p_j}}
\end{align*}
$$

ここで、最後の和の部分を見ると、\\(\frac{x}{p_j} < \frac{x}{p_a} \leq x^{2/3}\\)であるから、\\(x^{2/3}\\)まで篩などで前計算しておけば\\(\app{P_2}{x,a}\\)が計算できる。

## \\(\phi\\)の計算

\\(\app{\phi}{x,a}\\)はそのままでは計算が難しく、\\(x\\)の部分または\\(a\\)の部分を小さくしたい。このとき次の展開公式が使える。

$$\app\phi{x,a} = \app\phi{x,a-1} - \app\phi{\frac{x}{p_a}, a-1}$$

この展開公式を適応するにあたって、次の指標を導入する。\\(\paren{n, b}\\)：\\(n\\)の素因数は相異なり、すべて\\(p_b\\)より大きい。
この\\(\paren{n, b}\\)は、展開された項\\(\paren{-1}^r\app\phi{\frac{x}{n}, b}\\)に対応している。ただし、\\(r\\)は\\(n\\)の素因数の数。

実は、展開中にはじめて\\(\paren{n, b}\\)が以下のいずれかの条件を満たすようなとき、対応する項が簡単に計算できる。

1. \\(n \le x^{1/3} \land b = 0\\)
1. \\(n > x^{1/3}\\)

論文中では1に対応するのをordinary leaf、2に対応するのをspecial leafと呼んでいる。

### ordinary leafについて

このとき対応する項は\\(\paren{-1}^r\app\phi{\frac{x}{n}, 0} = \paren{-1}^r\floor{\frac{x}{n}}\\)である。\\(r\\)や\\(n\\)の素因数が重複しないなどを考えると、
\\(\app\mu{n}\floor{\frac{x}{n}}\\)を足せばよいことがわかる。ただし\\(\app\mu{\star}\\)はMöbius関数である。

### special leafについて

\\(\paren{n, b}\\)の構成から\\(b+1 < a\\)であり、

- \\(n = m p_{b+1}\\)
- \\(m \le x^{1/3} < n\\)
- \\(m\\)の最小素因数は\\(p_{b+1}\\)より大きい

となるような\\(m\\)が存在する。このとき対応する項は、\\(\paren{-1}^r\app\phi{\frac{x}{n}, b} = -\app\mu{m}\app\phi{\frac{x}{m p_{b+1}}, b}\\)となる。
\\(\frac{x}{n} < x^{2/3}\\)であるので、これも\\(x^{2/3}\\)まで篩いながら計算することができる（詳しくは後述）。

# 実装

前計算として、\\(\app{f}{x}\\)を\\(x\\)の最小素因数として、\\(\app{f}{n}, \app\mu{n}\\)を\\(n \leq x^{1/3}\\)の範囲で求める。
このとき、\\(x^{1/3}\\)以下の素数や\\(a\\)なども求めておく。
この時点でordinary leafの計算が可能である。
前計算を利用して、\\(x^{2/3}\\)まで篩っていくのだが、この途中経過を利用してspecial leafの計算を進めていく。
具体的には、\\(b\\)の昇順に見て、\\(p_b\\)で篩い終わった後、
上の条件を満たすような\\(m\\)に対して\\(-\app\mu{m}\app\phi{\frac{x}{m p_{b+1}}, b}\\)を足し合わせればよい。
ただし、\\(\phi\\)の計算にはfenwick tree(BIT)を用いる。
最終的に\\(x^{2/3}\\)まで篩い終わると、\\(P_2\\)の計算ができるようになるので、符号に注意して、計算する。

# 高速化 :TODO

# おわりに
いかがでしたか？たぶんLucy DPとかの方がいいと思う（今更）。
