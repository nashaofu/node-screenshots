# 📸 node-screenshots

`node-screenshots` 是一个原生的 node.js 截图库，支持 Mac、Windows 和 Linux 系统，且无需任何依赖。

[English](README.md) | 简体中文

## 支持矩阵

### 操作系统

| 操作系统       | node14 | node16 | node18 |
| -------------- | ------ | ------ | ------ |
| Windows x64    | ✓      | ✓      | ✓      |
| Windows x32    | ✓      | ✓      | ✓      |
| Windows arm64  | ✓      | ✓      | ✓      |
| macOS x64      | ✓      | ✓      | ✓      |
| macOS arm64    | ✓      | ✓      | ✓      |
| Linux x64 gnu  | ✓      | ✓      | ✓      |
| Linux x64 musl | ✓      | ✓      | ✓      |

## 示例

```ts
const fs = require("fs");
const { Screenshots } = require("node-screenshots");

let capturer = Screenshots.fromPoint(100, 100);

console.log(capturer, capturer.id);

// 同步截图
let image = capturer.captureSync();
fs.writeFileSync("./a.png", image);

// 异步截图
capturer.capture().then((data) => {
    console.log(data);
    fs.writeFileSync(`${capturer.id}.png`, data);
});

// 获取所有屏幕截图
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

-   `Screenshots.fromPoint(x, y)`: 获取指定坐标的截图
-   `Screenshots.all()`: 获取所有截图
-   `screenshots.capture()`: 异步截取全屏
-   `screenshots.captureSync()`: 同步截取全屏
-   `screenshots.captureArea(x, y, width, height)`: 异步截取指定区域
-   `screenshots.captureAreaSync(x, y, width, height)`: 同步截取指定区域

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
