---
title: "NixOSにしました"
date: 2024-06-14T04:00:14+09:00
draft: false
tags:
- Linux
- Nix
summary: "NixOSをインストール・セットアップした"
---

# 現在の環境

TODO: スクショにする
```
% nix run nixpkgs#fastfetch
          ▗▄▄▄       ▗▄▄▄▄    ▄▄▄▖             qitoy@nixos
          ▜███▙       ▜███▙  ▟███▛             -----------
           ▜███▙       ▜███▙▟███▛              OS: NixOS 24.11.20240719.1d9c2c9 (Vicuna) aarch64
            ▜███▙       ▜██████▛               Host: Apple MacBook Air (M1, 2020)
     ▟█████████████████▙ ▜████▛     ▟▙         Kernel: Linux 6.9.9-asahi
    ▟███████████████████▙ ▜███▙    ▟██▙        Uptime: 9 hours, 31 mins
           ▄▄▄▄▖           ▜███▙  ▟███▛        Packages: 572 (nix-system), 688 (nix-user)
          ▟███▛             ▜██▛ ▟███▛         Shell: zsh 5.9
         ▟███▛               ▜▛ ▟███▛          Display (eDP-1): 2560x1600 @ 60Hz (as 2048x1280) [Built-in]
▟███████████▛                  ▟██████████▙    WM: Hyprland (Wayland)
▜██████████▛                  ▟███████████▛    Cursor: macOS-Monterey (22px)
      ▟███▛ ▟▙               ▟███▛             Terminal: alacritty 0.13.2
     ▟███▛ ▟██▙             ▟███▛              Terminal Font: MoralerspaceNeonNF (12.5pt)
    ▟███▛  ▜███▙           ▝▀▀▀▀               CPU: Apple M1 (8) @ 3.20 GHz
    ▜██▛    ▜███▙ ▜██████████████████▛         GPU: Apple M1 [Integrated]
     ▜▛     ▟████▙ ▜████████████████▛          Memory: 2.37 GiB / 7.37 GiB (32%)
           ▟██████▙       ▜███▙                Swap: 0 B / 8.00 GiB (0%)
          ▟███▛▜███▙       ▜███▙               Disk (/): 54.74 GiB / 313.43 GiB (17%) - ext4
         ▟███▛  ▜███▙       ▜███▙              Local IP (wlp1s0f0): 192.168.3.36/24
         ▝▀▀▀    ▀▀▀▀▘       ▀▀▀▘              Battery: 100% [AC Connected]
                                               Power Adapter: 30W
                                               Locale: ja_JP.UTF-8

                                               ████████████████████████
                                               ████████████████████████
```

# NixOSのインストール手順

1. [nixos-apple-silicon](https://github.com/tpwrules/nixos-apple-silicon)のインストールにはUSBメモリが必要なので新しく購入。
1. Fedora Asahi Remix上でdocを見ながらISOを作成し、USBメモリに`dd`で書き込み。
1. MacOSに移動し、uninstallerを叩いてFedoraを削除（パーティションは残るのでそこにインストールする）。
1. installerを叩きUEFI environment onlyをインストールし、シャットダウン。USBメモリを刺し、起動。
1. docに書いてあるように設定し、NixOSをインストール。シャットダウンし、USBメモリを抜く。
1. 起動して無事にインストールされていることを確認。

インストール後にメモリ抜かずに再起動してメモリからbootしてあれ？になってた（1敗）。

# dotfilesに乗せる

とりあえず`configuration.nix`と`hardware-configuration.nix`をdotfilesのflakeに乗せてcommit(fe21ec1)。apple-silicon-supportをinputsに指定した(14f1695)。最初Wi-Fiはiwdで設定していたが、大学Wi-Fiに繋げられずいつものwpa_supplicantに変更。その際`networking.wireless.networks`を設定しなくてはならないが、生パスワードなどは乗せられないのでこれを非公開にする必要がある。flakesの仕様上、untrackなファイルは使えないので、[agenix](https://github.com/ryantm/agenix)を使用し、暗号化した状態でaddすることにした(7ec7698)。

後は細かい修正などをして今に至る。

# おわりに

NixOS、めちゃ癖強いけどNixの恩恵全部享受してて最高
