#!/bin/bash

# YAMLテンプレートファイルのパス
TEMPLATE_FILE="wg_tmpl.yml.j2"
# 出力するYAMLファイルのパス
OUTPUT_FILE="auto-gen/wireguard_setup.yml"

# 引数からnode_countを取得（デフォルトは3）
NODE_COUNT=${1:-3}

# テンプレートファイルの存在確認
if [[ ! -f "$TEMPLATE_FILE" ]]; then
  echo "Error: Template file '$TEMPLATE_FILE' not found."
  exit 1
fi

# YAMLを生成
echo "Generating YAML file with node_count=${NODE_COUNT}..."
cat "$TEMPLATE_FILE" | sed "s/{{ node_count }}/$NODE_COUNT/g" > "$OUTPUT_FILE"

if [[ $? -eq 0 ]]; then
  echo "YAML file generated: $OUTPUT_FILE"
else
  echo "Error: Failed to generate YAML file."
  exit 1
fi
