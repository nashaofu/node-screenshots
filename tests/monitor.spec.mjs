import test from "ava";
import { Monitor } from "../index.js";

test("Monitor.all()", (t) => {
  let monitors = Monitor.all();
  t.true(monitors.length >= 0);
});

test("Monitor.fromPoint(100, 100)", (t) => {
  let monitor = Monitor.fromPoint(100, 100);
  t.true(monitor !== null);
  t.true(monitor instanceof Monitor);
});

test("Monitor.fromPoint(-1000, -1000)", (t) => {
  let monitor = Monitor.fromPoint(-1000, -1000);
  t.true(monitor === null);
});

test("monitor.captureImage()", async (t) => {
  let monitor = Monitor.fromPoint(100, 100);
  let image = await monitor.captureImage();
  t.true(image.width > 0);
  t.true(image.height > 0);
});

test("monitor.captureImageSync()", (t) => {
  let monitor = Monitor.fromPoint(100, 100);
  let image = monitor.captureImageSync();
  t.true(image.width > 0);
  t.true(image.height > 0);
});
