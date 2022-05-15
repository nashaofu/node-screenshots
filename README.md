# node-screenshots

Zero-dependent. A native nodejs screenshots library for Mac、Windows、Linux.

## Support matrix

### Operating Systems

| Operating Systems | node12 | node14 | node16 |
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
let all = Screenshots.all()

all.forEach(capturer => {
  console.log({
    id: capturer.id,
    x: capturer.x,
    y: capturer.y,
    width: capturer.width,
    height: capturer.height,
    scale: capturer.scale,
    rotation: capturer.rotation
  })
  capturer.captureSync()
})

// 从屏幕id获取截图，id可以通过 `Screenshots.all()` 获取，也可以通过electron的 `screen.getAllDisplays()` 获取
let sc = Screenshots.fromDisplay(71)
```

## API

- `Screenshots.fromPoint(x, y)`: 从指定坐标获取屏幕
- `Screenshots.fromDisplay(displayId)`: 从屏幕 id 获取屏幕
- `Screenshots.all()`: 获取所有屏幕
- `screenshots.capture()`: 异步截图
- `screenshots.captureSync()`: 同步截图
