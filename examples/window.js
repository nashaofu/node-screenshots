console.time("require");
const { Window } = require("..");
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
    const windows = runWithTime(() => Window.all(), "Window.all()");

    for (let item of windows) {
        console.log(
            "Window:",
            item.id,
            item.appName,
            item.title,
            item.currentMonitor,
            item.x,
            item.y,
            item.width,
            item.height,
            item.isMinimized,
            item.isMaximized
        );

        let image = runWithTime(
            () => item.captureImageSync(true),
            "item.captureImageSync(true);"
        );
        writeFile(`window-${item.id}.png`, image);

        let captureImagePromise = runWithTime(
            () => item.captureImage(),
            "item.captureImage()"
        );
        console.log("item captureImagePromise:", captureImagePromise);

        console.time(`await ${item.id} captureImagePromise`);
        const data = await captureImagePromise;
        console.timeLog(`await ${item.id} captureImagePromise`);
        writeFile(`window-async-${item.id}.png`, data);
    }
}

main();
