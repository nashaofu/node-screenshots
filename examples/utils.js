const fs = require('fs')
const path = require('path')

async function saveImage(filename, buffer) {
  if (!fs.existsSync('target')) {
    fs.mkdirSync('target')
  }

  fs.writeFileSync(path.join(__dirname, '../target', filename), buffer)
}

function runWithTime(fn, label) {
  console.time(label)
  let result = fn()
  console.timeEnd(label)
  return result
}

module.exports = {
  saveImage,
  runWithTime,
}
