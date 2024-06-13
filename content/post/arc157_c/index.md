---
title: "YY Square"
date: 2023-11-28T10:07:42+09:00
draft: false
math: true
summary: "愚直DPを多項式(あるいはFPS)を用いて高速化した"
---

# 書く

$$
\global\def\ans{\operatorname{ans}}
\global\def\dd#1{\frac{\operatorname{d}}{\operatorname{d}#1}}
$$

\\( dp[i][j][k] \coloneqq s[i][j] \\)までに"YY"を\\( k \\)回通った  
とすると初期値\\( dp[1][1][0] = 1 \\)、遷移は
```rust
if s[i][j] == 'Y' && s[i+1][j] == 'Y' {
    dp[i+1][j][k+1] += dp[i][j][k]
} else {
    dp[i+1][j][k] += dp[i][j][k]
}
```
(\\(j\\)のときも同様)となり答えは
$$ \ans = \sum _{k=0} ^{h+w-2} dp[h][w][k] k^2 $$
である。これは\\(O(h w (h+w))\\)なので自明にTLEする。

ここで3次元配列を持つかわりに、2次元配列に多項式を載せることを考える。多項式は以下のように定義する。
$$ f _{i, j}(x) \coloneqq \sum _{k=0} ^{h+w-2} dp[i][j][k] x^k $$
この多項式を用いると、
$$
\ans = \left. \dd{x}\left( x \dd{x} f _{h, w}(x) \right) \right| _{x=1}
= f _{h, w}'(1) + f _{h, w}''(1)
$$
となることがわかる。これより、\\(f\\)の2階微分まで持てばよさそうだとわかる。ここで、遷移を見る。

```rust
dp[i+1][j][k+1] += dp[i][j][k]
```
に対応するのは、
$$ f _{i+1, j}(x) \xleftarrow{+} x f _{i, j}(x) $$
なので
$$ f _{i+1, j}'(x) \xleftarrow{+} x f _{i, j}'(x) + f _{i, j}(x) $$
$$ f _{i+1, j}''(x) \xleftarrow{+} x f _{i, j}''(x) + 2 f _{i, j}'(x) $$
より、
$$ f _{i+1, j}(1) \xleftarrow{+} f _{i, j}(1) $$
$$ f _{i+1, j}'(1) \xleftarrow{+} f _{i, j}'(1) + f _{i, j}(1) $$
$$ f _{i+1, j}''(1) \xleftarrow{+} f _{i, j}''(1) + 2 f _{i, j}'(1) $$

ただし\\(\xleftarrow{+}\\)は`+=`のこととする。\\(j\\)の方を無視しているようだが、線形性などから問題ない。遷移の下の方は自明なので省く。

したがって多項式を陽に持たなくても\\(x=1\\)のときのみ管理すればいいことがわかった。計算量は\\(O(h w)\\)まで落ち、ACすることができる。

# おわりに

KaTeXやったけどあんま好きじゃない
