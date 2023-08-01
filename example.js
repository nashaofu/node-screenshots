console.time("require");
const { Screenshots } = require(".");
console.timeEnd("require");
const fs = require("fs");
const path = require("path");

function writeFile(filename, buf) {
    if (!fs.existsSync("target")) {
        fs.mkdirSync("target");
    }

    fs.writeFileSync(path.join("target", filename), buf);
}

console.time("fromPoint");
let capturer = Screenshots.fromPoint(100, 100);
console.timeEnd("fromPoint");

console.time("display");
console.log(capturer, capturer.id);
console.timeEnd("display");

console.time("captureSync");
let image = capturer.captureSync(true);
console.timeEnd("captureSync");
console.log(image);
writeFile("temp-a.png", image);
writeFile("temp-a2.png", image);

console.time("captureAsync");
console.time("captureAsync task");
let captureAsync = capturer.capture();
console.timeEnd("captureAsync");
console.log("captureAsync", captureAsync);

captureAsync.then((data) => {
    console.timeEnd("captureAsync task");
    console.log(data);
    writeFile(`temp-${capturer.id}.png`, data);
});

console.time("Screenshots.all()");
let all = Screenshots.all();
console.timeEnd("Screenshots.all()");

all.forEach((capturer) => {
    // capturer.captureSync()
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

capturer.captureArea(300, 300, 300, 300, false).then((buffer) => {
    writeFile(`temp-captureArea-${capturer.id}.png`, buffer);
});
