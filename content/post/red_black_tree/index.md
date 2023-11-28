---
title: "赤黒木を書いた（遅い）"
date: 2023-10-22T16:27:59+09:00
draft: false
---

# これは何

永続遅延伝播反転可能赤黒木

<!--more-->

# 参考文献

全体的に
[永続赤黒木を実装した時のメモ](http://blog.mitaki28.info/1447078746296/)

実装の部分で
[packer-jp/persistent-lazy-rbtrees](https://github.com/packer-jp/persistent-lazy-rbtrees/blob/main/rs/persistent_lazy_rbtree/src/lib.rs)

# 実装方針

とりあえず遅延伝播や反転については考えないものとして
