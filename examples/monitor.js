console.time("require");
const { Monitor } = require("..");
console.timeEnd("require");
const fs = require("fs");
const path = require("path");

function writeFile(filename, buf) {
    if (!fs.existsSync("target")) {
        fs.mkdirSync("target");
    }

    fs.writeFileSync(path.join("target", filename), buf);
}

function runWithTime(fn, label) {
    console.time(label);
    let result = fn();
    console.timeEnd(label);
    return result;
}

async function main() {
    const monitors = runWithTime(() => Monitor.all(), "Monitor.all()");

    monitors.forEach((item) => {
        console.log(
            "Monitor:",
            item.id,
            item.name,
            [item.x, item.y, item.width, item.height],
            item.rotation,
            item.scaleFactor,
            item.frequency,
            item.isPrimary
        );
    });

    let monitor = runWithTime(
        () => Monitor.fromPoint(100, 100),
        "Monitor.fromPoint(100, 100)"
    );

    let image = runWithTime(
        () => monitor.captureImageSync(true),
        "monitor.captureImageSync(true);"
    );
    writeFile(`temp-monitor-${monitor.id}.png`, image);

    let captureImagePromise = runWithTime(
        () => monitor.captureImage(),
        "monitor.captureImage()"
    );
    console.log("Monitor captureImagePromise:", captureImagePromise);

    console.time("await captureImagePromise");
    const data = await captureImagePromise;
    console.timeLog("await captureImagePromise");
    writeFile(`temp-monitor-async-${monitor.id}.png`, data);
}

main();
