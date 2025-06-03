#!/bin/bash

# path-finder-jk バイナリダウンロード＆セットアップスクリプト

# 設定可能な変数
VERSION="v1.0.9"
ARCHITECTURE="aarch64-apple-darwin"  # macOS用のデフォルト

# スクリプトのディレクトリを取得
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# GitHub リリースURL
GITHUB_REPO="k0f1sh/path-finder-jk"
BINARY_NAME="path-finder"
ARCHIVE_NAME="${BINARY_NAME}-${ARCHITECTURE}.tar.gz"
DOWNLOAD_URL="https://github.com/${GITHUB_REPO}/releases/download/${VERSION}/${ARCHIVE_NAME}"

echo "path-finder-jk ${VERSION} をダウンロードしています..."
echo "アーキテクチャ: ${ARCHITECTURE}"
echo "ダウンロードURL: ${DOWNLOAD_URL}"

# 既存のバイナリとアーカイブを削除
if [ -f "${SCRIPT_DIR}/${BINARY_NAME}" ]; then
    echo "既存のバイナリを削除しています..."
    rm "${SCRIPT_DIR}/${BINARY_NAME}"
fi

if [ -f "${SCRIPT_DIR}/${ARCHIVE_NAME}" ]; then
    echo "既存のアーカイブを削除しています..."
    rm "${SCRIPT_DIR}/${ARCHIVE_NAME}"
fi

# バイナリをダウンロード
echo "バイナリをダウンロードしています..."
if curl -L -o "${SCRIPT_DIR}/${ARCHIVE_NAME}" "${DOWNLOAD_URL}"; then
    echo "ダウンロード完了"
else
    echo "ダウンロードに失敗しました"
    exit 1
fi

# アーカイブを展開
echo "アーカイブを展開しています..."
if tar -xzf "${SCRIPT_DIR}/${ARCHIVE_NAME}" -C "${SCRIPT_DIR}"; then
    echo "展開完了"
else
    echo "展開に失敗しました"
    exit 1
fi

# アーカイブファイルを削除（クリーンアップ）
rm "${SCRIPT_DIR}/${ARCHIVE_NAME}"

# バイナリに実行権限を付与
chmod +x "${SCRIPT_DIR}/${BINARY_NAME}"

echo "セットアップ完了！"
echo "バイナリは ${SCRIPT_DIR}/${BINARY_NAME} に配置されました。"
echo "実行するには: ${SCRIPT_DIR}/${BINARY_NAME}"

