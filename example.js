console.time("require");
const { Screenshots } = require(".");
console.timeEnd("require");
const fs = require("fs");
const path = require("path");

function writeFile(name, buf) {
    if (!fs.existsSync("target")) {
        fs.mkdirSync("target");
    }

    fs.writeFileSync(path.join('target', name), buf);
}

console.time("fromPoint");
let capturer = Screenshots.fromPoint(100, 100);
console.timeEnd("fromPoint");

console.time("display");
console.log(capturer, capturer.id);
console.timeEnd("display");

console.time("captureSync");
let image = capturer.captureSync();
console.timeEnd("captureSync");
writeFile("./temp-a.bmp", image.toBmp());
writeFile("./temp-a.png", image.toPng());
writeFile("./temp-a.jpeg", image.toJpeg());
writeFile("./temp-rgba", image.toRgba());

console.time("captureAsync");
console.time("captureAsync task");
let captureAsync = capturer.capture();
console.timeEnd("captureAsync");
console.log("captureAsync", captureAsync);

captureAsync.then((image) => {
    console.timeEnd("captureAsync task");
    console.log(image);

    writeFile(`temp-${capturer.id}.png`, image.toPng());
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

capturer.captureArea(300, 300, 300, 300).then((image) => {
    writeFile(`temp-captureArea-${capturer.id}.png`, image.toPng());
});
