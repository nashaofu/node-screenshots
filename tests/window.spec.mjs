import test from "ava";
import { Window } from "../index.js";

test("Window.all()", (t) => {
  let windows = Window.all();
  t.true(windows.length >= 0);
});
