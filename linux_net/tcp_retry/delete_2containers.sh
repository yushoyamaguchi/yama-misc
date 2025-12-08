#!/usr/bin/env bash
set -euo pipefail

# ===== 設定（起動スクリプトと同じ名前を使う） =====
NETWORK_NAME="${NETWORK_NAME:-netshoot-net}"
C1_NAME="${C1_NAME:-netshoot-a}"
C2_NAME="${C2_NAME:-netshoot-b}"

echo "[*] 削除対象:"
echo "  コンテナ: ${C1_NAME}, ${C2_NAME}"
echo "  ネットワーク: ${NETWORK_NAME}"
echo

# ===== Docker コマンド確認 =====
if ! command -v docker >/dev/null 2>&1; then
  echo "ERROR: docker コマンドが見つかりません。" >&2
  exit 1
fi

# ===== コンテナ削除関数 =====
delete_container () {
  local name="$1"
  if docker ps -a --format '{{.Names}}' | grep -Fxq "${name}"; then
    echo "[*] コンテナ ${name} を停止して削除します..."
    docker rm -f "${name}" >/dev/null
    echo "    -> 削除完了"
  else
    echo "[*] コンテナ ${name} は存在しません。スキップ。"
  fi
}

delete_container "${C1_NAME}"
delete_container "${C2_NAME}"
echo

# ===== ネットワーク削除 =====
if docker network inspect "${NETWORK_NAME}" >/dev/null 2>&1; then
  echo "[*] ネットワーク ${NETWORK_NAME} を削除します..."
  docker network rm "${NETWORK_NAME}" >/dev/null
  echo "    -> 削除完了"
else
  echo "[*] ネットワーク ${NETWORK_NAME} は存在しません。スキップ。"
fi

echo
echo "[*] 完了しました。"
