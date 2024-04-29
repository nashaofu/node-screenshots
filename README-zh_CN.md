# 📸 node-screenshots

`node-screenshots` 是一个基于[XCap](https://github.com/nashaofu/xcap)的原生的 node.js 截图库，支持 Mac、Windows 和 Linux 系统，且无需任何依赖。 支持截图与视频录制（待实现）。

[English](README.md) | 简体中文

## 支持矩阵

### 操作系统

| 操作系统       | node16 | node18 | node20 |
| -------------- | ------ | ------ | ------ |
| Windows x64    | ✓      | ✓      | ✓      |
| Windows x32    | ✓      | ✓      | ✓      |
| Windows arm64  | ✓      | ✓      | ✓      |
| macOS x64      | ✓      | ✓      | ✓      |
| macOS arm64    | ✓      | ✓      | ✓      |
| Linux x64 gnu  | ✓      | ✓      | ✓      |
| Linux x64 musl | ✓      | ✓      | ✓      |

## 示例

### Monitor

```ts
const fs = require("fs");
const { Monitor } = require("node-screenshots");

let monitor = Monitor.fromPoint(100, 100);

console.log(monitor, monitor.id);

let image = monitor.captureImageSync();
fs.writeFileSync(`${monitor.id}-sync.png`, image.toPngSync());

monitor.captureImage().then((data) => {
    console.log(data);
    fs.writeFileSync(`${monitor.id}.jpeg`, data.toJpegSync());
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

```ts
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
    fs.writeFileSync(`${item.id}-sync.bmp`, image.toBmpSync());

    item.captureImage().then(async (data) => {
        console.log(data);
        let newImage = await data.crop(10, 10, 10, 10);
        fs.writeFileSync(`${item.id}.png`, await newImage.toPng());
    });
});
```

## API

完整的 TypeScript 类型定义：[index.d.ts](./index.d.ts)

### Monitor

-   `static all(): Array<Monitor>`：获取所有监视器
-   `static fromPoint(x: number, y: number): Monitor | null`：根据指定的坐标获取监视器
-   `captureImageSync(): Image`：同步捕获图像
-   `captureImage(): Promise<Image>`：异步捕获图像

### Window

-   `static all(): Array<Window>`：获取所有窗口
-   `captureImageSync(): Image`：同步捕获图像
-   `captureImage(): Promise<Image>`：异步捕获图像

### Image

-   `cropSync(x: number, y: number, width: number, height: number): Image`：同步裁剪图像
-   `crop(x: number, y: number, width: number, height: number): Promise<Image>`：异步裁剪图像
-   `toPngSync(copyOutputData?: boolean | undefined | null): Buffer`：同步转换为 PNG 格式
-   `toPng(copyOutputData?: boolean | undefined | null): Promise<Buffer>`：异步转换为 PNG 格式
-   `toJpegSync(copyOutputData?: boolean | undefined | null): Buffer`：同步转换为 JPEG 格式
-   `toJpeg(copyOutputData?: boolean | undefined | null): Promise<Buffer>`：异步转换为 JPEG 格式
-   `toBmpSync(copyOutputData?: boolean | undefined | null): Buffer`：同步转换为 BMP 格式
-   `toBmp(copyOutputData?: boolean | undefined | null): Promise<Buffer>`：异步转换为 BMP 格式
-   `toRawSync(copyOutputData?: boolean | undefined | null): Buffer`：同步转换为原始格式（RGBA 数据）
-   `toRaw(copyOutputData?: boolean | undefined | null): Promise<Buffer>`：异步转换为原始格式（RGBA 数据）

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
