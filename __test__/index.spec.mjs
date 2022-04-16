import test from 'ava'

import { Screenshots } from '../index.js'

test('ScrennCapture.all()', t => {
  let all = Screenshots.all()
  t.true(all.length >= 0)
})
