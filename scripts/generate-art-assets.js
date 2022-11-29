// @ts-check

const path = require("node:path");
const sharp = require("sharp");

const inputSvgsPrefix = ["node_modules", "@mdi", "svg", "svg"];

/** @type {string[]} */
const inputSvgs = [
  path.join(...inputSvgsPrefix, "microsoft-xbox.svg"),
  path.join(...inputSvgsPrefix, "nintendo-switch.svg"),
  path.join(...inputSvgsPrefix, "sony-playstation.svg"),
  path.join(...inputSvgsPrefix, "steam.svg"),
];

/** @type {(inputPath: string) => Promise<void>} */
async function svgToPng(inputPath) {
  const inputBaseName = path.basename(inputPath);
  const inputExtension = path.extname(inputBaseName);
  const outputFormat = "png";
  const outputExtension = "." + outputFormat;
  const outputBaseName = inputBaseName.substring(0, inputBaseName.length - inputExtension.length) + outputExtension;
  const outputPath = path.join("src", "assets", outputBaseName);
  await sharp(inputPath)
    .resize(1024, 1024)
    .flatten({ background: { r: 255, g: 255, b: 255 } })
    .png()
    .toFile(outputPath);
}

/** @type {() => Promise<void>} */
async function main() {
  for (const inputPath of inputSvgs) {
    await svgToPng(inputPath);
  }
}

main().catch(console.error);
