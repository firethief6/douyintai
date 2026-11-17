import assert from "node:assert/strict";
import { filterIds } from "../src/lib/filter.ts";

const ids = ["v", "g"];
const itemOf = (id) => id === "v"
  ? { kind: "video", createTime: 1793692800 }
  : id === "g" ? { kind: "gallery", createTime: 1778774400 } : undefined;
const f = (p = {}) => ({ kind: p.kind ?? "all", from: p.from ?? 0, to: p.to ?? 0, limit: p.limit ?? 0 });

assert.deepEqual(filterIds(ids, itemOf, f()), ["v", "g"]);
assert.deepEqual(filterIds(ids, itemOf, f({ kind: "video" })), ["v"]);
assert.deepEqual(filterIds(ids, itemOf, f({ kind: "gallery" })), ["g"]);
assert.deepEqual(filterIds(ids, itemOf, f({ from: 1780272000 })), ["v"]);
assert.deepEqual(filterIds(ids, itemOf, f({ to: 1780272000 })), ["g"]);
assert.deepEqual(filterIds(ids, itemOf, f({ limit: 1 })), ["v"]);
assert.deepEqual(filterIds(ids, itemOf, f({ kind: "gallery", from: 1780272000 })), []);
assert.deepEqual(filterIds(ids, itemOf, f({ kind: "video", limit: 1 })), ["v"]);
console.log("filter: 8/8");
