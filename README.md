# node-screenshots

Zero-dependent. A native nodejs screenshots library for Mac、Windows、Linux.

## Support matrix

### Operating Systems

| Operating Systems | node14 | node16 | node18 |
| ----------------- | ------ | ------ | ------ |
| Windows x64       | ✓      | ✓      | ✓      |
| Windows x32       | ✓      | ✓      | ✓      |
| Windows arm64     | ✓      | ✓      | ✓      |
| macOS x64         | ✓      | ✓      | ✓      |
| macOS arm64       | ✓      | ✓      | ✓      |
| Linux x64 gnu     | ✓      | ✓      | ✓      |
| Linux x64 musl    | ✓      | ✓      | ✓      |

## example

```ts
const fs = require('fs')
const { Screenshots } = require('node-screenshots')

let capturer = Screenshots.fromPoint(100, 100)

console.log(capturer, capturer.id)

// 同步截图
let image = capturer.captureSync()
fs.writeFileSync('./a.png', image)

// 异步获取截图
capturer.capture().then(data => {
  console.log(data)
  fs.writeFileSync(`${capturer.id}.png`, data)
})

// 获取所有屏幕截图
let all = Screenshots.all() ?? []

all.forEach(capturer => {
  console.log({
    id: capturer.id,
    x: capturer.x,
    y: capturer.y,
    width: capturer.width,
    height: capturer.height,
    rotation: capturer.rotation,
    scaleFactor: capturer.scaleFactor,
    isPrimary: capturer.isPrimary
  })
  capturer.captureSync()
})

```

## API

- `Screenshots.fromPoint(x, y)`: 从指定坐标获取屏幕
- `Screenshots.all()`: 获取所有屏幕
- `screenshots.capture()`: 异步截取全屏
- `screenshots.captureSync()`: 同步截取全屏
- `screenshots.captureArea(x, y, width, height)`: 异步截取屏幕指定区域
- `screenshots.captureAreaSync(x, y, width, height)`: 同步截取屏幕指定区域

## Linux requirements

On Linux, you need to install `libxcb`、`libxrandr`、`dbus`

Debian/Ubuntu:

```sh
apt-get install libxcb1 libxrandr2 libdbus-1-3
```

Alpine:

```sh
apk add libxcb libxrandr dbus
```
