import test from "ava";
import { Monitor } from "../index.js";

test("Monitor.all()", (t) => {
    let monitors = Monitor.all();
    t.true(monitors.length == 1);
    t.true(typeof monitors[0].id === "number");
});

test("Monitor.fromPoint(100, 100)", (t) => {
    let monitor = Monitor.fromPoint(100, 100);
    t.true(monitor !== null);
});

test("Monitor.fromPoint(-1000, -1000)", (t) => {
    let monitor = Monitor.fromPoint(-1000, -1000);
    t.true(monitor === null);
});

test("monitor.captureImage()", async (t) => {
    let monitor = Monitor.fromPoint(100, 100);
    let buffer = await monitor.captureImage(false);
    t.true(buffer !== null);
});

test("monitor.captureImageSync()", (t) => {
    let monitor = Monitor.fromPoint(100, 100);
    let buffer = monitor.captureImageSync(true);
    t.true(buffer !== null);
});
