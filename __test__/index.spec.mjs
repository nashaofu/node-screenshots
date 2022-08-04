import test from 'ava'
import { Screenshots } from '../index.js'

test('Screenshots.all()', t => {
  let all = Screenshots.all()
  t.true(all.length == 1)
  t.true(typeof all[0].id === 'number')
})

test('Screenshots.fromPoint(100, 100)', t => {
  let screenshots = Screenshots.fromPoint(100, 100)
  t.true(screenshots !== null)
})

test('Screenshots.fromPoint(-10, -10)', t => {
  let screenshots = Screenshots.fromPoint(-10, -10)
  t.true(screenshots === null)
})

test('Screenshots.fromDisplay(0)', t => {
  let screenshots = Screenshots.fromDisplay(0)
  t.true(screenshots === null)
})

test('screenshots.capture()', async t => {
  let screenshots = Screenshots.fromPoint(100, 100)
  let buffer = await screenshots.capture()
  t.true(buffer !== null)
})

test('screenshots.captureSync()', t => {
  let screenshots = Screenshots.fromPoint(100, 100)
  let buffer = screenshots.captureSync()
  t.true(buffer !== null)
})

test('screenshots.captureArea(0, 0, 10, 10)', async t => {
  let screenshots = Screenshots.fromPoint(100, 100)
  let buffer = await screenshots.captureArea(0, 0, 10, 10)
  t.true(buffer !== null)
})

test('screenshots.captureAreaSync(0, 0, 10, 10)', t => {
  let screenshots = Screenshots.fromPoint(100, 100)
  let buffer = screenshots.captureAreaSync(0, 0, 10, 10)
  t.true(buffer !== null)
})
