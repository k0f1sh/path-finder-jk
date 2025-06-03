#!/bin/bash

# path-finder 実行スクリプト

# スクリプトのディレクトリを取得
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BINARY_PATH="${SCRIPT_DIR}/path-finder"

# バイナリが存在するかチェック
if [ ! -f "${BINARY_PATH}" ]; then
    echo "エラー: path-finder バイナリが見つかりません。"
    echo "先に setup.sh を実行してバイナリをダウンロードしてください。"
    echo ""
    echo "実行方法: ${SCRIPT_DIR}/setup.sh"
    exit 1
fi

# バイナリに実行権限があるかチェック
if [ ! -x "${BINARY_PATH}" ]; then
    echo "バイナリに実行権限を付与しています..."
    chmod +x "${BINARY_PATH}"
fi

echo "path-finder を実行します"
echo "========================================"

# readlineを有効にして、パス補完機能を利用
read -e -p "スキャン対象のディレクトリパスを入力してください: " scan_path

# 入力が空の場合はカレントディレクトリを使用
if [ -z "$scan_path" ]; then
    scan_path="."
    echo "パスが入力されていないため、カレントディレクトリ (.) を使用します。"
fi

# チルダ（~）をホームディレクトリに展開
scan_path="${scan_path/#\~/$HOME}"

# パスが存在するかチェック
if [ ! -d "$scan_path" ]; then
    echo "エラー: 指定されたパス '$scan_path' は存在しないか、ディレクトリではありません。"
    exit 1
fi

# 絶対パスに変換
scan_path=$(cd "$scan_path" && pwd)

echo ""
echo "スキャン対象: $scan_path"
echo "========================================"
echo "path-finderを実行しています..."

# 出力ファイルのパスを設定
output_file="${SCRIPT_DIR}/../result.json"

# path-finder を実行し、結果をファイルに出力
"${BINARY_PATH}" scan-directory --json "$scan_path" > "$output_file"

# 実行完了メッセージ
if [ $? -eq 0 ]; then
    echo "実行完了: 結果は $output_file に保存されました。"
else
    echo "エラー: path-finderの実行に失敗しました。"
    exit 1
fi