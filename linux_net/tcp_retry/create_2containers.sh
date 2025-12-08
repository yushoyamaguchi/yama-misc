#!/usr/bin/env bash
set -euo pipefail

# ===== 設定（必要なら環境変数で上書き可） =====
IMAGE="${IMAGE:-nicolaka/netshoot}"
NETWORK_NAME="${NETWORK_NAME:-netshoot-net}"
C1_NAME="${C1_NAME:-netshoot-a}"
C2_NAME="${C2_NAME:-netshoot-b}"

echo "[*] 使うイメージ:      ${IMAGE}"
echo "[*] Dockerネットワーク: ${NETWORK_NAME}"
echo "[*] コンテナ名:        ${C1_NAME}, ${C2_NAME}"
echo

# ===== Docker が入っているかざっくりチェック =====
if ! command -v docker >/dev/null 2>&1; then
  echo "ERROR: docker コマンドが見つかりません。Docker Desktop for Mac をインストールして下さい。" >&2
  exit 1
fi

# ===== ネットワーク作成（なければ） =====
if docker network inspect "${NETWORK_NAME}" >/dev/null 2>&1; then
  echo "[*] ネットワーク ${NETWORK_NAME} は既に存在します。"
else
  echo "[*] ネットワーク ${NETWORK_NAME} を作成します..."
  docker network create "${NETWORK_NAME}"
fi
echo

# ===== コンテナ起動関数 =====
run_netshoot () {
  local name="$1"
  # 既に同名コンテナがある場合はスキップ
  if docker ps -a --format '{{.Names}}' | grep -Fxq "${name}"; then
    echo "[*] コンテナ ${name} は既に存在します。起動状態にしておきます..."
    docker start "${name}" >/dev/null 2>&1 || true
  else
    echo "[*] コンテナ ${name} を起動します..."
    docker run -d \
      --privileged \
      --sysctl net.ipv4.tcp_retries2=5 \
      --name "${name}" \
      --hostname "${name}" \
      --network "${NETWORK_NAME}" \
      --cap-add=NET_ADMIN \
      --cap-add=NET_RAW \
      --rm \
      "${IMAGE}" \
      sleep infinity
  fi
}

run_netshoot "${C1_NAME}"
run_netshoot "${C2_NAME}"
echo

echo "[*] 起動中の netshoot コンテナ:"
docker ps --filter "name=${C1_NAME}" --filter "name=${C2_NAME}"

echo
echo "==== 疎通確認例 ===="
echo "別ターミナルで以下を実行して下さい:"
echo "  docker exec -it ${C1_NAME} bash"
echo "  # コンテナ内で:"
echo "  ping ${C2_NAME}"
echo
echo "逆方向:"
echo "  docker exec -it ${C2_NAME} bash"
echo "  ping ${C1_NAME}"
echo
echo "[*] コンテナ停止例:"
echo "  docker stop ${C1_NAME} ${C2_NAME}"