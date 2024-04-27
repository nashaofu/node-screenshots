const { Monitor } = require("..");
const { saveImage, runWithTime } = require("./utils");

async function main() {
    const monitors = Monitor.all();

    let monitor = Monitor.fromPoint(100, 100);

    let image = monitor.captureImageSync();
    console.log("Image:", image.width, image.height);

    saveImage(`image-${monitor.id}.jpeg`, await image.toJpeg());
    saveImage(`image-${monitor.id}.bmp`, await image.toBmp());
    saveImage(`image-${monitor.id}.png`, await image.toPng());

    console.log(await image.toRaw());

    saveImage(`image-sync-${monitor.id}.jpeg`, image.toJpegSync());
    saveImage(`image-sync-${monitor.id}.bmp`, image.toBmpSync());
    saveImage(`image-sync-${monitor.id}.png`, image.toPngSync());

    console.log(await image.toRawSync());

    saveImage(
        `image-crop-${monitor.id}.png`,
        (await image.crop(100, 100, 300, 300)).toPngSync()
    );
    saveImage(
        `image-crop-sync-${monitor.id}.png`,
        image.cropSync(100, 100, 300, 300).toPngSync()
    );

    saveImage(`image-crop-original-${monitor.id}.png`, image.toPngSync());

    runWithTime(() => image.toJpegSync(), "image.toJpegSync()");
    runWithTime(() => image.toBmpSync(), "image.toBmpSync()");
    runWithTime(() => image.toPngSync(), "image.toPngSync()");
    runWithTime(
        () => image.cropSync(100, 100, 300, 300),
        "image.cropSync(100, 100, 300, 300)"
    );
}

main();
