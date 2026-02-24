<h1 align="center">LogBee 🐝</h1>

![](screenshot.gif)

## 🚀 LogBee とは？

LogBeeは、JSON形式のログファイルを閲覧するためのTUI（ターミナルUI）ツールです。
内部に **DuckDB** を搭載しているため、数万件のログに対しても標準的なSQL構文（WHERE句）を使って、瞬時にフィルタリングやソートが行えます。

## ✨ 主な機能

- 🔍 **SQLライクなフィルタリング**: `WHERE` 句の構文で複雑な条件抽出が可能

## 📦 インストール方法

### Windows

1. [Releases](https://github.com/masotyu/LogBee/releases) ページから最新の `LogBee-setup.exe` をダウンロードします。
2. インストーラーを実行し、指示に従ってインストールしてください。

### Ubuntu / Debian (Linux)

1. [Releases](https://github.com/masotyu/LogBee/releases) ページから `.deb` パッケージをダウンロードします。
2. 以下のコマンドでインストールします：

```bash
sudo dpkg -i logbee_*.deb
```

### Fedora / RHEL / CentOS

1. [Releases](https://github.com/masotyu/LogBee/releases) ページから最新の `.rpm` ファイルをダウンロードします。
2. 以下のコマンドを実行してインストールします：

```bash
sudo dnf install ./logbee-*.rpm
```

## 📖 使い方

### 1. LogBee の起動

ターミナルから、閲覧したい JSON ログファイルを引数に指定して実行します。

```bash
logbee your-log-file.json
```

## ⌨️ キーバインド

キー | アクション
---|------
i | 検索モード: SQL の WHERE 句を入力（例: level > 30）
Enter | フォーカス切替: ログリスト ↔ 詳細ビュー（黄色枠）
j / k | 移動 / スクロール: リストの選択、または詳細エリアの縦スクロール
Tab | ソート列切替: タイムスタンプ、レベル、メッセージなどで並び替え
s | ソート順切替: 昇順 (▲) / 降順 (▼)
n / p | ページ移動: 次の 100 件 / 前の 100 件
Esc | キャンセル: 検索終了、または詳細エリアからリストに戻る
q | 終了

## 🔍 クエリの例 (Query エリア)

DuckDB の構文がそのまま使えます：

```
level >= 40 (WARN 以上を抽出)

msg LIKE '%timeout%' (メッセージに timeout を含む)

hostname = 'prod-api-01' (特定のホストに絞り込み)
```