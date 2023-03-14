---
title: "ヤジリン解説"
date: 2023-03-14T04:44:25+09:00
draft: false
---

# これは何

https://puzsq.logicpuzzle.app/puzzle/106302 の解説をするよ

<!--more-->

# 解説

## 分割充填

分割充填をします。
{{< figure src="yajilin.png" alt="分割充填" width="320" height="320" >}}
すると左端の2が確定します。次にこの2つの領域に注目します。
{{< figure src="yajilin2.png" alt="領域" width="320" height="320" >}}
この領域にはそれぞれ1つしか黒マスが入りません。なぜなら2つ入るとすると、即座にハタンしてしまうからです。
{{< figure src="yajilin3.png" alt="ハタン" width="320" height="320" >}}
したがって右の2マスに黒マスが1つ入ることがわかります。ここで左側に入れると分割充填を満たさないので、右側に確定します。分割充填からさらに1つ確定します。次にこの3行の下2行に注目します。分割充填から、1あるいは2のマスに黒マスが入ります。
{{< figure src="yajilin4.png" alt="領域2" width="320" height="320" >}}
1のマスに黒マスを入れると、即座にハタンします。
{{< figure src="yajilin5.png" alt="ハタン2" width="320" height="320" >}}
よって2のマスに入ります。さらに少し進めます。
ここで1のマスか2のマスかの2択になりますが、
{{< figure src="yajilin6.png" alt="2択" width="320" height="320" >}}
2のマスに入れると即座にハタンします。
{{< figure src="yajilin7.png" alt="ハタン3" width="320" height="320" >}}
よって1のマスになり、最後の2択も右側に置くとハタンするので、左側に決まり、分割充填パートが終わりました。
{{< figure src="yajilin8.png" alt="ハタン4" width="320" height="320" >}}
{{< figure src="yajilin9.png" alt="分割充填おわり" width="320" height="320" >}}

## 121

下側のこの領域に注目すると、121の形になっていることがわかります。
{{< figure src="yajilin10.png" alt="領域3" width="320" height="320" >}}
偶奇より、この領域は線が1本だけ通ることがわかり、従ってナナメに横断することがわかります。
{{< figure src="yajilin11.png" alt="ナナメ" width="320" height="320" >}}
このうち左上・右下のパターンでは、小ループ禁に反するので、左下・右上のパターンになります。これで121パートが終わりました。
{{< figure src="yajilin12.png" alt="小ループ禁" width="320" height="320" >}}
{{< figure src="yajilin13.png" alt="121おわり" width="320" height="320" >}}

## のこりもの

あとはそこまで難しいものはありません。右上の単純仮定と左下の壁際に気をつければ完成です。
{{< figure src="yajilin14.png" alt="のこりもの" width="320" height="320" >}}
{{< figure src="yajilin15.png" alt="ハタン5" width="320" height="320" >}}
{{< figure src="yajilin16.png" alt="壁際" width="320" height="320" >}}
{{< figure src="yajilin17.png" alt="完成" width="320" height="320" >}}

# あとがき

結構はしょったけど書くの大変

