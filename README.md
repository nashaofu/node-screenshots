# 📸 node-screenshots

`node-screenshots` is a native node.js screenshot library based on [XCap](https://github.com/nashaofu/xcap), It supports Mac, Windows, and Linux systems without any dependencies. `node-screenshots` supports screenshot and video recording (to be implemented).

English | [简体中文](README-zh_CN.md)

## Support Matrix

### Operating System

| Operating System | node14 | node16 | node18 | node20 |
| ---------------- | ------ | ------ | ------ | ------ |
| Windows x64      | ✓      | ✓      | ✓      | ✓      |
| Windows x32      | ✓      | ✓      | ✓      | ✓      |
| Windows arm64    | ✓      | ✓      | ✓      | ✓      |
| macOS x64        | ✓      | ✓      | ✓      | ✓      |
| macOS arm64      | ✓      | ✓      | ✓      | ✓      |
| Linux x64 gnu    | ✓      | ✓      | ✓      | ✓      |
| Linux x64 musl   | ✓      | ✓      | ✓      | ✓      |

## Example

### Monitor

```ts
const fs = require("fs");
const { Monitor } = require("node-screenshots");

let monitor = Monitor.fromPoint(100, 100);

console.log(monitor, monitor.id);

let image = monitor.captureImageSync();
fs.writeFileSync(`${monitor.id}-sync.png`, image);

monitor.captureImage().then((data) => {
    console.log(data);
    fs.writeFileSync(`${monitor.id}.png`, data);
});

let monitors = Monitor.all();

monitors.forEach((capturer) => {
    console.log({
        id: capturer.id,
        x: capturer.x,
        y: capturer.y,
        width: capturer.width,
        height: capturer.height,
        rotation: capturer.rotation,
        scaleFactor: capturer.scaleFactor,
        isPrimary: capturer.isPrimary,
    });
});
```

### Window

```js
const fs = require("fs");
const { Window } = require("node-screenshots");

let windows = Window.all();

windows.forEach((item) => {
    console.log({
        id: item.id,
        x: item.x,
        y: item.y,
        width: item.width,
        height: item.height,
        rotation: item.rotation,
        scaleFactor: item.scaleFactor,
        isPrimary: item.isPrimary,
    });

    // Synchronous capture
    let image = item.captureImageSync();
    fs.writeFileSync(`${item.id}-sync.png`, image);

    // Asynchronous capture
    item.captureImage().then((data) => {
        console.log(data);
        fs.writeFileSync(`${item.id}.png`, data);
    });
});
```

## API

TypeScript type definition: [index.d.ts](./index.d.ts)

### Monitor

-   `Monitor.fromPoint(x, y)`: Get a monitor from the specified coordinates
-   `Monitor.all()`: Get all monitor
-   `monitor.captureImageSync(copyOutputData)`: Synchronously capture
-   `monitor.captureImage(copyOutputData)`: Asynchronously capture

### Window

-   `Window.all()`: Get all window
-   `window.captureImageSync(copyOutputData)`: Synchronously capture
-   `window.captureImage(copyOutputData)`: Asynchronously capture

`copyOutputData`: pass related parameters in electron, otherwise electron will crash, nodejs does not pass or passes false, performance will be better, for more information refer to https://github.com/napi-rs/napi-rs/issues/1346

## Linux System Requirements

On Linux, you need to install `libxcb`, `libxrandr`, and `dbus`.

Debian / Ubuntu:

```sh
apt-get install libxcb1 libxrandr2 libdbus-1-3
```

Alpine:

```sh
apk add libxcb libxrandr dbus
```
