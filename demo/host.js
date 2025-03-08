const { spawn } = require("child_process");

const vendorId = "18003";
const productId = "4";

const qmkHid = spawn("qmk-hid", [
  "--vendor-id",
  vendorId,
  "--product-id",
  productId,
]);

qmkHid.stdout.on("data", (data) => {
  console.log("Received:", data.toString().trim());
});

setTimeout(() => {
  const message = "Hello, QMK!\n"; // Ensure newline is included
  console.log("Sending:", message.trim());
  qmkHid.stdin.write(message);
}, 2000);
