import assert from "node:assert/strict";
import { groupFromRaw, groupOf, groupQuery, knownVideoRatio, stepIn, videoRatio } from "../src/lib/stage-nav.ts";

assert.equal(groupOf(new URLSearchParams()), null);
assert.deepEqual(groupOf(new URLSearchParams("series=1001")), { kind: "series", id: "1001" });
assert.deepEqual(groupOf(new URLSearchParams("mix=m%201")), { kind: "mix", id: "m 1" });
assert.deepEqual(groupOf(new URLSearchParams("series=%20&mix=2")), { kind: "mix", id: "2" });
assert.equal(groupOf(new URLSearchParams("series=1&mix=2"))?.kind, "series");
for (const group of [{ kind: "series", id: "a b" }, { kind: "mix", id: "m/1" }]) {
  const query = new URLSearchParams(groupQuery(group));
  assert.deepEqual(groupOf(query), group);
}
assert.equal(groupQuery({ kind: "series", id: "a b" }), "?series=a%20b");
assert.equal(groupQuery({ kind: "mix", id: "m/1" }), "?mix=m%2F1");
assert.deepEqual(groupFromRaw({ native_series: { series_id: 8 } }), { kind: "series", id: "8" });
assert.deepEqual(groupFromRaw({ series_info: { series_id: "s2" } }), { kind: "series", id: "s2" });
assert.deepEqual(groupFromRaw({ series_info: { series_id: "m2", content_sub_type: 3 } }), { kind: "mix", id: "m2" });
assert.deepEqual(groupFromRaw({ mix_info: { mix_id: "m3" } }), { kind: "mix", id: "m3" });
assert.equal(groupFromRaw({ series_info: { series_id: "" } }), null);
assert.equal(groupFromRaw(null), null);

const ids = ["a", "b", "c"];
assert.deepEqual(stepIn(ids, "b", 1), { id: "c", index: 1, reason: "ok" });
assert.deepEqual(stepIn(ids, "b", -1), { id: "a", index: 1, reason: "ok" });
assert.deepEqual(stepIn(ids, "a", -1), { id: null, index: 0, reason: "first" });
assert.deepEqual(stepIn(ids, "c", 1), { id: null, index: 2, reason: "last" });
assert.deepEqual(stepIn(ids, "x", 1), { id: null, index: -1, reason: "missing" });
assert.deepEqual(stepIn(["a"], "a", 1), { id: null, index: 0, reason: "last" });
assert.deepEqual(stepIn([], "a", -1), { id: null, index: -1, reason: "missing" });

assert.equal(videoRatio({ video: { width: 1920, height: 1080 } }), 16 / 9);
assert.equal(videoRatio({ video: { width: 1080, height: 1920 } }), 9 / 16);
assert.equal(videoRatio({ video: { bit_rate: [{ play_addr: { width: 1280, height: 720 } }] } }), 16 / 9);
assert.equal(videoRatio({ video: { width: 1080, height: 1920, bit_rate: [{ play_addr: { width: 1280, height: 720 } }] } }), 9 / 16);
assert.equal(videoRatio({ video: { width: 0, height: 0, bit_rate: [{ play_addr: { width: 720, height: 1280 } }] } }), 9 / 16);
assert.equal(videoRatio({ video: { width: -1920, height: 1080 } }), 9 / 16);
assert.equal(videoRatio({ video: { width: "1920", height: "1080" } }), 16 / 9);
assert.equal(videoRatio({}), 9 / 16);
assert.equal(videoRatio(null), 9 / 16);
assert.equal(knownVideoRatio({ video: { width: 1920, height: 1080 } }), 16 / 9);
assert.equal(knownVideoRatio({ video: { width: 0, height: 0 } }), 0);
assert.equal(knownVideoRatio({}), 0);
assert.equal(knownVideoRatio(null), 0);
console.log("stage nav: passed");
