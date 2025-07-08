#!/bin/bash

# 使用方法チェック
if [ $# -ne 1 ]; then
    echo "Usage: $0 <source_dir_name>"
    echo "Example: $0 built_cilium_dsr"
    exit 1
fi

# 引数からコピー元ディレクトリ名を取得
SOURCE_DIR="$1"
SOURCE_FILE="./$SOURCE_DIR/kind-image/Dockerfile"

# コピー元ファイルの存在チェック
if [ ! -f "$SOURCE_FILE" ]; then
    echo "Error: Source file not found: $SOURCE_FILE"
    exit 1
fi

# コピー先のパスをリストで定義
DEST_PATHS=(
    "release_cilium_dsr/kind-image/Dockerfile"
    "bgp-cplane-dev-v4/kind-image/Dockerfile"
    "bgp-cplane-dev-v6/kind-image/Dockerfile"
    "built_cilium_wg/kind-image/Dockerfile"
    "cilium_lb/cl1/kind-image/Dockerfile"
    "cilium_wg_native_keepalive/kind-image/Dockerfile"
    "built_cilium_dsr/kind-image/Dockerfile"
    "built_v6_dsr/kind-image/Dockerfile"
    "local-build-example/kind-image/Dockerfile"
    "built_cilium_lb/kind-image/Dockerfile"
    "../observability/kind-nginx-lb/kind-image/Dockerfile"
)

# 各パスにファイルをコピー
for dest in "${DEST_PATHS[@]}"; do
    cp "$SOURCE_FILE" "$dest"
    echo "Copied to: $dest"
done
