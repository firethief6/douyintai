#!/usr/bin/env bash
# 用仓库根 .env 里的 token 操作 origin。按 .env.example 创建 .env。
# 用法：./scripts/push.sh fetch [分支]
#       FORCE=1 ./scripts/push.sh <分支|标签>   # 覆盖远端分支，标签先删再推
#       ./scripts/push.sh <分支|标签> [...]
set -euo pipefail

ROOT="$(git -C "$(dirname "$0")" rev-parse --show-toplevel)"
ENV_FILE="$ROOT/.env"
cd "$ROOT"

[[ -f "$ENV_FILE" ]] || { echo "缺少 $ENV_FILE，按 .env.example 创建" >&2; exit 1; }
set -a; . "$ENV_FILE"; set +a
[[ -n "${PUBLIC_TOKEN:-}" && "${PUBLIC_TOKEN}" != "xxx" ]] || { echo "先在 $ENV_FILE 填 PUBLIC_TOKEN" >&2; exit 1; }
git remote get-url origin >/dev/null 2>&1 || { echo "没有 origin 远端" >&2; exit 1; }

# gh 提供 credential helper，token 只在环境变量里，不进 URL 和 argv。
GIT=(git -c credential.helper= -c credential.helper='!gh auth git-credential')
MODE="${1:-}"
shift || true

case "$MODE" in
  fetch)
    BRANCH="${1:-$(git rev-parse --abbrev-ref HEAD)}"
    GH_TOKEN="$PUBLIC_TOKEN" "${GIT[@]}" fetch --force origin "$BRANCH:refs/remotes/origin/$BRANCH" --tags
    printf '已更新 origin/%s\n' "$BRANCH"
    ;;
  ''|-h|--help)
    echo "用法：$0 fetch [分支] | <分支|标签> [...]" >&2
    exit 1
    ;;
  *)
    for ref in "$MODE" "$@"; do
      # 标签没有远端跟踪引用，改为先删远端标签再推，不依赖 force。
      if [[ "${FORCE:-0}" == "1" ]] && git show-ref --verify --quiet "refs/tags/$ref"; then
        tag_sha="$(git rev-parse "refs/tags/$ref^{}")"
        if [[ "$tag_sha" != "$(git rev-parse HEAD)" && "${TAG_ANY:-0}" != "1" ]]; then
          printf '标签 %s 指向 %s，不是 HEAD %s。先 git tag -f %s HEAD，或 TAG_ANY=1 跳过。\n' \
            "$ref" "${tag_sha:0:7}" "$(git rev-parse --short HEAD)" "$ref" >&2
          exit 1
        fi
        GH_TOKEN="$PUBLIC_TOKEN" "${GIT[@]}" push origin ":refs/tags/$ref" || true
        GH_TOKEN="$PUBLIC_TOKEN" "${GIT[@]}" push origin "$ref"
      elif [[ "${FORCE:-0}" == "1" ]]; then
        GH_TOKEN="$PUBLIC_TOKEN" "${GIT[@]}" push --force-with-lease origin "$ref"
      else
        GH_TOKEN="$PUBLIC_TOKEN" "${GIT[@]}" push origin "$ref"
      fi
    done
    ;;
esac
