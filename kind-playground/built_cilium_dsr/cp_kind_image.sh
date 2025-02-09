#!/bin/bash

# コピー元のファイル
SOURCE_FILE="built_cilium_dsr/kind-image/Dockerfile"

# コピー先のパスをリストで定義
DEST_PATHS=(
    "release_cilium_dsr/kind-image/Dockerfile"
    "bgp-cplane-dev-v4/kind-image/Dockerfile"
    "bgp-cplane-dev-v6/kind-image/Dockerfile"
    "built_cilium_wg/kind-image/Dockerfile"
    "cilium_lb/cl1/kind-image/Dockerfile"
    "cilium_wg_native_keepalive/kind-image/Dockerfile"
    "../observability/kind-nginx-lb/kind-image/Dockerfile"
)

# 各パスにファイルをコピー
for dest in "${DEST_PATHS[@]}"; do
    cp "$SOURCE_FILE" "$dest"
    echo "Copied to: $dest"
done