# 📸 node-screenshots

`node-screenshots` is a native node.js screenshot library that supports Mac, Windows, and Linux systems without any dependencies.

English | [简体中文](README-zh_CN.md)

## Support Matrix

### Operating System

| Operating System | node14 | node16 | node18 |
| ---------------- | ------ | ------ | ------ |
| Windows x64      | ✓      | ✓      | ✓      |
| Windows x32      | ✓      | ✓      | ✓      |
| Windows arm64    | ✓      | ✓      | ✓      |
| macOS x64        | ✓      | ✓      | ✓      |
| macOS arm64      | ✓      | ✓      | ✓      |
| Linux x64 gnu    | ✓      | ✓      | ✓      |
| Linux x64 musl   | ✓      | ✓      | ✓      |

## Example

```ts
const fs = require("fs");
const { Screenshots } = require("node-screenshots");

let capturer = Screenshots.fromPoint(100, 100);

console.log(capturer, capturer.id);

// Synchronous capture
let image = capturer.captureSync();
fs.writeFileSync("./a.png", image);

// Asynchronous capture
capturer.capture().then((data) => {
    console.log(data);
    fs.writeFileSync(`${capturer.id}.png`, data);
});

// Get all screen captures
let all = Screenshots.all() ?? [];

all.forEach((capturer) => {
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
    capturer.captureSync();
});
```

## API

-   `Screenshots.fromPoint(x, y)`: Get a screenshot from the specified coordinates
-   `Screenshots.all()`: Get all screenshots
-   `screenshots.capture()`: Asynchronously capture full screen
-   `screenshots.captureSync()`: Synchronously capture full screen
-   `screenshots.captureArea(x, y, width, height)`: Asynchronously capture the specified area
-   `screenshots.captureAreaSync(x, y, width, height)`: Synchronously capture the specified area

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
