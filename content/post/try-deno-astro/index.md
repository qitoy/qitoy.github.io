---
title: "DenoでAstroをしようとした"
date: 2024-01-10T08:40:05+09:00
draft: false
---

# これは何

現時点では完全にはできなかった

<!--more-->

# 要因

エラーログを見る
```
08:41:48 [ERROR] Cannot read properties of undefined (reading 'env')
  Stack trace:
    at /path/to/project/node_modules/.deno/astro@4.1.1/node_modules/astro/dist/content/runtime
.js, <anonymous>:54:65
    [...] See full stack trace in the browser, or rerun with --verbose.
```

この`env`は何だと色々調査してみたところ、Node.jsにはある`process`というグローバル変数がDenoには存在しないのでエラーを吐いているということがわかった。

なおこの件に関してissue[#20886](https://github.com/denoland/deno/issues/20886)が立てられており、ワンチャン今後対応されて使えるようになるかも

# おわりに

いかがでしたか？
