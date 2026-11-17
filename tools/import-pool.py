#!/usr/bin/env python3
"""把 douyin 仓库的账号池导成随包 JSONL（只留运行期字段，去绝对路径）。"""
import json, pathlib, sys

src = pathlib.Path(sys.argv[1] if len(sys.argv) > 1 else pathlib.Path.home() / ".cache/douyin/accounts")
out = pathlib.Path(sys.argv[2] if len(sys.argv) > 2 else "crates/dycore/resources/pool.jsonl")
FIELDS = ("device_id", "iid", "cdid", "device_token", "brand", "model", "resolution", "dpi",
          "android_id", "wifi_mac", "bt_mac", "used", "search_state", "status")

rows = []
for p in sorted(src.glob("*.json")):
    try:
        d = json.loads(p.read_text())
    except Exception:
        continue
    if not d.get("device_id"):
        continue
    rows.append({k: d[k] for k in FIELDS if d.get(k)})

out.parent.mkdir(parents=True, exist_ok=True)
out.write_text("\n".join(json.dumps(r, ensure_ascii=False) for r in rows) + "\n")
print(f"{len(rows)} 条 -> {out} ({out.stat().st_size} B)")
