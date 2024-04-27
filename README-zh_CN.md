# 📸 node-screenshots

`node-screenshots` 是一个基于[XCap](https://github.com/nashaofu/xcap)的原生的 node.js 截图库，支持 Mac、Windows 和 Linux 系统，且无需任何依赖。 支持截图与视频录制（待实现）。

[English](README.md) | 简体中文

## 支持矩阵

### 操作系统

| 操作系统 | node14 | node16 | node18 | node20 |
| ---------------- | ------ | ------ | ------ | ------ |
| Windows x64      | ✓      | ✓      | ✓      | ✓      |
| Windows x32      | ✓      | ✓      | ✓      | ✓      |
| Windows arm64    | ✓      | ✓      | ✓      | ✓      |
| macOS x64        | ✓      | ✓      | ✓      | ✓      |
| macOS arm64      | ✓      | ✓      | ✓      | ✓      |
| Linux x64 gnu    | ✓      | ✓      | ✓      | ✓      |
| Linux x64 musl   | ✓      | ✓      | ✓      | ✓      |

## 示例

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

-   Window

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

    let image = item.captureImageSync();
    fs.writeFileSync(`${item.id}-sync.png`, image);

    item.captureImage().then((data) => {
        console.log(data);
        fs.writeFileSync(`${item.id}.png`, data);
    });
});
```

## API

TypeScript 类型定义: [index.d.ts](./index.d.ts)

### Monitor

-   `Monitor.fromPoint(x, y)`: 获取指定坐标的屏幕
-   `Monitor.all()`: 获取所有屏幕
-   `monitor.captureImageSync(copyOutputData)`: 同步截图
-   `monitor.captureImage(copyOutputData)`: 异步截图

### Window

-   `Window.all()`: 获取所有窗口
-   `window.captureImageSync(copyOutputData)`: 同步截图
-   `window.captureImage(copyOutputData)`: 异步截图

`copyOutputData`: electron 中传递传递相关参数，否则 electron 会崩溃，nodejs 不传或者传递 false，性能会更好，详细信息参考 https://github.com/napi-rs/napi-rs/issues/1346

## Linux 系统需求

在 Linux 上，你需要安装 `libxcb`、`libxrandr` 和 `dbus`。

Debian / Ubuntu：

```sh
apt-get install libxcb1 libxrandr2 libdbus-1-3
```

Alpine：

```sh
apk add libxcb libxrandr dbus
```
