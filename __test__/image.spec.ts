import test from 'ava'
import { Monitor, Image } from '../index.js'

test('Image', (t) => {
  let monitor = Monitor.fromPoint(100, 100)

  let image = monitor.captureImageSync()

  t.true(image instanceof Image)
  t.true(image.cropSync(10, 10, 10, 10) instanceof Image)
  t.true(Buffer.isBuffer(image.toJpegSync()))
  t.true(Buffer.isBuffer(image.toBmpSync()))
  t.true(Buffer.isBuffer(image.toPngSync()))
  t.true(Buffer.isBuffer(image.toRawSync()))
})
