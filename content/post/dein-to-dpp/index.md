---
title: "dein.vimからdpp.vimに段階的に移行する"
date: 2023-10-27T11:05:18+09:00
tags:
- Vim
- dpp.vim
draft: false
summary: "実際のコード付き"
---

# before

```vim
if &compatible
  set nocompatible               " Be iMproved
endif

if has('nvim')
  lua if vim.loader then vim.loader.enable() end
endif

" dein install
let s:cache = expand('~/.cache')
if !isdirectory(s:cache)
  call mkdir(s:cache, 'p')
endif

let s:dein = s:cache .. '/dein/repos/github.com/Shougo/dein.vim'
if !isdirectory(s:dein)
  execute '!git clone https://github.com/Shougo/dein.vim' s:dein
endif
execute 'set runtimepath^=' .. s:dein

let g:dein#auto_recache = v:true
let g:dein#install_progress_type = 'floating'
let g:dein#install_check_diff = v:true
let g:dein#install_github_api_token = $DEIN_INSTALL_GITHUB_API_TOKEN

let $VIM_DIR = expand('~/.vim')
let $VIM_TOMLS = $VIM_DIR .. '/' .. 'tomls'
let $VIM_HOOKS = $VIM_DIR .. '/' .. 'hooks'

let g:dein#inline_vimrcs = ['$VIM_DIR/settings.vim']

if dein#load_state(s:cache .. '/dein')
  " Required:
  call dein#begin(s:cache .. '/dein', expand('<sfile>'))

  call dein#load_toml('$VIM_TOMLS/dein.toml', {'lazy': 0})
  call dein#load_toml('$VIM_TOMLS/dein_lazy.toml', {'lazy': 1})
  if has('nvim')
    call dein#load_toml('$VIM_TOMLS/nvim.toml', {'lazy': 1})
  else
    call dein#load_toml('$VIM_TOMLS/vim.toml', {'lazy': 1})
  endif
  call dein#load_toml('$VIM_TOMLS/ddc.toml', {'lazy': 0})
  call dein#load_toml('$VIM_TOMLS/ddu.toml', {'lazy': 0})

  " Required:
  call dein#end()
  call dein#save_state()
endif

" Required:
filetype plugin indent on
syntax enable

" If you want to install not installed plugins on startup.
if dein#check_install()
	call dein#install()
endif
```

# 移行する

## 参考

[リファレンス実装](https://github.com/Shougo/shougo-s-github)

## 書き換え

### インストールスクリプトを関数化

```diff
-let s:dein = s:cache .. '/dein/repos/github.com/Shougo/dein.vim'
-if !isdirectory(s:dein)
-  execute '!git clone https://github.com/Shougo/dein.vim' s:dein
-endif
-execute 'set runtimepath^=' .. s:dein
+function InitPlugin(plugin)
+  let dir = s:cache .. '/dein/repos/github.com/' .. a:plugin
+  if !dir->isdirectory()
+    execute '!git clone https://github.com/' .. a:plugin dir
+  endif
+  execute 'set runtimepath^=' .. dir
+endfunction
+
+call InitPlugin('Shougo/dein.vim')
```

### （やってない場合）`augroup vimrc`をしておく

```diff
+augroup vimrc
+  autocmd!
+augroup END
```

### `dpp#min#load_state()`を配置

ソース読んだ感じだと無害そうなので一旦置く、ただし`dpp#make_state()`の副作用はデカいのでコメントアウト

```diff
+if dpp#min#load_state(s:cache .. '/dein')
+  echohl WarningMsg | echomsg 'begin make state' | echohl NONE
+
+  for s:plugin in [
+  \ 'Shougo/dpp-ext-installer',
+  \ 'Shougo/dpp-ext-toml',
+  \ 'Shougo/dpp-protocol-git',
+  \ 'vim-denops/denops.vim',
+  \]
+    call InitPlugin(s:plugin)
+  endfor
+
+  " NOTE: need manual load
+  runtime! plugin/denops.vim
+
+  " autocmd vimrc User DenopsReady
+  " \ call dpp#make_state(s:cache .. '/dein', '$VIM_HOOKS/dpp.ts'->expand())
+  autocmd vimrc User Dpp:makeStatePost
+  \ echohl WarningMsg | echomsg 'end make state' | echohl NONE
+else
+  autocmd vimrc BufWritePost *.lua,*.vim,*.toml,*.ts,vimrc,.vimrc
+  \ call dpp#check_files()
+endif
```

### configを書く

`init.vim`にあるdeinの設定を書けばよい。

```typescript
import {
  BaseConfig,
  ConfigArguments,
  ConfigReturn,
  Plugin,
} from "https://deno.land/x/dpp_vim@v0.0.7/types.ts";
import { fn } from "https://deno.land/x/dpp_vim@v0.0.7/deps.ts";

// dpp-ext-toml
type Toml = {
  hooks_file?: string;
  ftplugins?: Record<string, string>;
  plugins: Plugin[];
};

// dpp-ext-lazy
type LazyMakeStateResult = {
  plugins: Plugin[];
  stateLines: string[];
};

export class Config extends BaseConfig {
  override async config(args: ConfigArguments): Promise<ConfigReturn> {
    const hasNvim = args.denops.meta.host === "nvim";

    // setting inline_vimrcs
    const inlineVimrcs = ["$VIM_DIR/settings.vim"];

    args.contextBuilder.setGlobal({
      inlineVimrcs,
      extParams: {
        installer: {
          checkDiff: true,
        },
      },
      protocols: ["git"],
      protocolParams: {
        git: {
          enablePartialClone: true,
        },
      },
    });

    const [context, options] = await args.contextBuilder.get(args.denops);

    // toml plugins
    const tomls: Toml[] = [];
    // non-lazy
    for (
      const toml of [
        "$VIM_TOMLS/dein.toml",
        "$VIM_TOMLS/ddc.toml",
        "$VIM_TOMLS/ddu.toml",
      ]
    ) {
      tomls.push(
        await args.dpp.extAction(
          args.denops,
          context,
          options,
          "toml",
          "load",
          {
            path: toml,
            options: {
              lazy: false,
            },
          },
        ) as Toml,
      );
    }
    // lazy
    for (
      const toml of [
        "$VIM_TOMLS/dein_lazy.toml",
        hasNvim ? "$VIM_TOMLS/nvim.toml" : "$VIM_TOMLS/vim.toml",
      ]
    ) {
      tomls.push(
        await args.dpp.extAction(
          args.denops,
          context,
          options,
          "toml",
          "load",
          {
            path: toml,
            options: {
              lazy: true,
            },
          },
        ) as Toml,
      );
    }

    // merge result
    const recordPlugins: Record<string, Plugin> = {};
    const ftplugins: Record<string, string> = {};
    const hooksFiles: string[] = [];
    for (const toml of tomls) {
      for (const plugin of toml.plugins) {
        recordPlugins[plugin.name] = plugin;
      }

      if (toml.ftplugins) {
        for (const filetype of Object.keys(toml.ftplugins)) {
          if (!ftplugins[filetype]) {
            ftplugins[filetype] = "";
          }
          // ftplugins[filetype] is not undefined
          ftplugins[filetype] += `\n${toml.ftplugins[filetype]}`;
        }
      }

      if (toml.hooks_file) {
        hooksFiles.push(toml.hooks_file);
      }
    }

    const lazyResult = await args.dpp.extAction(
      args.denops,
      context,
      options,
      "lazy",
      "makeState",
      {
        plugins: Object.values(recordPlugins),
      },
    ) as LazyMakeStateResult;

    // $VIM_DIR/init.vim
    // $VIM_DIR/settings.vim
    // $VIM_TOMLS/*,
    // $VIM_HOOKS/*,
    const checkFiles: string[] = [];
    checkFiles.push(
      ...await fn.globpath(
        args.denops,
        "$VIM_DIR",
        "*.vim",
        1,
        1,
      ) as unknown as string[],
    );
    checkFiles.push(
      ...await fn.globpath(
        args.denops,
        "$VIM_TOMLS",
        "*",
        1,
        1,
      ) as unknown as string[],
    );
    checkFiles.push(
      ...await fn.globpath(
        args.denops,
        "$VIM_HOOKS",
        "*",
        1,
        1,
      ) as unknown as string[],
    );

    return {
      checkFiles,
      ftplugins,
      hooksFiles,
      plugins: lazyResult.plugins,
      stateLines: lazyResult.stateLines,
    };
  }
}
```

### テスト

思い切って`dein`の設定を`if 0`するなどし、`dpp#make_state()`のコメントを外す

```diff
-if dein#load_state(s:cache .. '/dein')
+" if dein#load_state(s:cache .. '/dein')
+if 0
...
-  " autocmd vimrc User DenopsReady
-  " \ call dpp#make_state(s:cache .. '/dein', '$VIM_HOOKS/dpp.ts'->expand())
+  autocmd vimrc User DenopsReady
+  \ call dpp#make_state(s:cache .. '/dein', '$VIM_HOOKS/dpp.ts'->expand())
...
-if dein#check_install()
+" if dein#check_install()
+if 0
```

### バグ取り

自分の環境では
```
[denops] A 'deno' (g:denops#deno) is not executable. Denops requires executable Deno.
```
と出てきたので以下の行を追加

```diff
+let g:denops#deno = '~/.deno/bin/deno'->expand()
```

```
[denops] Failed to handle message 2,invoke,dispatch,dpp,makeState,/home/qitoy/.cache/dein,/home/qito
y/.vim/hooks/dpp.ts,nvim 0,function dpp#ext#lazy#_generate_dummy_mappings, line 7: Vim(let):E928: St
ring required
```

なんじゃこりゃ～～～

どうやら`on_map`は`(Dictionary) or (List) or (String)`だけど実装を見ると`String`はダメっぽいので修正

```diff
-on_map = '<Plug>(asterisk-'
+on_map = ['<Plug>(asterisk-']
```

### 仕上げ

問題無さそうだったら`dein.vim`関連の記述を削除してプラグインのインストールをする

最終的にこうなった

```vim
if &compatible
  set nocompatible               " Be iMproved
endif

if has('nvim')
  lua if vim.loader then vim.loader.enable() end
endif

augroup vimrc
  autocmd!
augroup END

let g:denops#deno = '~/.deno/bin/deno'->expand()

" dpp install
let s:cache = expand('~/.cache')
if !isdirectory(s:cache)
  call mkdir(s:cache, 'p')
endif

function InitPlugin(plugin)
  let dir = s:cache .. '/dpp/repos/github.com/' .. a:plugin
  if !dir->isdirectory()
    execute '!git clone https://github.com/' .. a:plugin dir
  endif
  execute 'set runtimepath^=' .. dir
endfunction

let $VIM_DIR = expand('~/.vim')
let $VIM_TOMLS = $VIM_DIR .. '/' .. 'tomls'
let $VIM_HOOKS = $VIM_DIR .. '/' .. 'hooks'

call InitPlugin('Shougo/dpp.vim')
" need to load lazy plugins
call InitPlugin('Shougo/dpp-ext-lazy')

if dpp#min#load_state(s:cache .. '/dpp')
  echohl WarningMsg | echomsg 'begin make state' | echohl NONE

  for s:plugin in [
  \ 'Shougo/dpp-ext-installer',
  \ 'Shougo/dpp-ext-toml',
  \ 'Shougo/dpp-protocol-git',
  \ 'vim-denops/denops.vim',
  \]
    call InitPlugin(s:plugin)
  endfor

  " NOTE: need manual load
  runtime! plugin/denops.vim

  autocmd vimrc User DenopsReady
  \ call dpp#make_state(s:cache .. '/dpp', '$VIM_HOOKS/dpp.ts'->expand())
  autocmd vimrc User Dpp:makeStatePost
  \ echohl WarningMsg | echomsg 'end make state' | echohl NONE
else
  autocmd vimrc BufWritePost *.lua,*.vim,*.toml,*.ts,vimrc,.vimrc
  \ call dpp#check_files()
endif

filetype plugin indent on
syntax enable
```

# おわりに

後は細かい調整とかをしよう
