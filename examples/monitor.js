console.time("require");
const { Monitor } = require("..");
console.timeEnd("require");
const { saveImage, runWithTime } = require("./utils");

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
    () => monitor.captureImageSync(),
    "monitor.captureImageSync();"
  );
  saveImage(`monitor-${monitor.id}.jpeg`, image.toJpegSync());

  let captureImagePromise = runWithTime(
    () => monitor.captureImage(),
    "monitor.captureImage()"
  );
  console.log("Monitor captureImagePromise:", captureImagePromise);

  console.time("await captureImagePromise");
  const image2 = await captureImagePromise;
  console.timeLog("await captureImagePromise");
  saveImage(`monitor-async-${monitor.id}.png`, image2.toPngSync());
}

main();
