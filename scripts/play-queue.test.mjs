import assert from "node:assert/strict";
import { advance, originOf, queueQuery, recordsHistory } from "../src/lib/play-list.ts";

const q = (s = "") => new URLSearchParams(s);

assert.equal(originOf(q(), false), "other");
assert.equal(originOf(q("from=history"), false), "history");
assert.equal(originOf(q("from=bogus"), false), "other");
assert.equal(originOf(q("series=9&from=history"), true), "series");
assert.equal(originOf(q("mix=8&from=feed"), true), "mix");
assert.equal(queueQuery("history"), "?from=history");
assert.equal(queueQuery("favorites"), "?from=favorites");
assert.equal(queueQuery("other"), "");
assert.equal(queueQuery("series", "9"), "?series=9");
assert.equal(queueQuery("mix", undefined, "m/1"), "?mix=m%2F1");
assert.equal(recordsHistory("history"), false);
assert.equal(recordsHistory("favorites"), false);
assert.equal(recordsHistory("feed"), true);
assert.equal(recordsHistory("series"), true);

const ids = ["a", "b", "c"];
assert.equal(advance(ids, "b", 1, false).id, "c");
assert.equal(advance(ids, "c", 1, false).reason, "last");
assert.equal(advance(ids, "a", -1, false).reason, "first");
assert.equal(advance(ids, "c", 1, true).id, "a");
assert.equal(advance(ids, "a", -1, true).id, "c");
assert.equal(advance(ids, "x", 1, true).reason, "missing");
assert.equal(advance([], "a", 1, true).reason, "missing");
console.log("play queue: passed");
