console.time('require')
const { Window } = require('..')
console.timeEnd('require')
const { saveImage, runWithTime } = require('./utils')

async function main() {
  const windows = runWithTime(() => Window.all(), 'Window.all()')

  for (let item of windows) {
    console.log(
      'Window:',
      item.id(),
      item.appName(),
      item.title(),
      item.currentMonitor(),
      item.x(),
      item.y(),
      item.width(),
      item.height(),
      item.isMinimized(),
      item.isMaximized(),
    )

    let image = runWithTime(() => item.captureImageSync(true), 'item.captureImageSync(true);')
    saveImage(`window-${item.id()}.bmp`, image.toBmpSync())

    let captureImagePromise = runWithTime(() => item.captureImage(), 'item.captureImage()')
    console.log('item captureImagePromise:', captureImagePromise)

    console.time(`await ${item.id()} captureImagePromise`)
    const image2 = await captureImagePromise
    console.timeLog(`await ${item.id()} captureImagePromise`)
    saveImage(`window-async-${item.id()}.bmp`, image2.toBmpSync())
  }
}

main()
