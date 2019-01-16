import 'edge-detection-wasm';
import wasmURL from 'edge-detection-wasm/edge_detection_wasm_bg.wasm';

let wasm;
const canvas = document.getElementById('canvas');
const videoEl = document.getElementById('video');
const ctx = canvas.getContext('2d');

function tick() {
  ctx.drawImage(videoEl, 0, 0, canvas.width, canvas.height);
  const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
  console.time('edge::detect');
  const data = wasm_bindgen.detect(
    imageData.data,
    canvas.width,
    canvas.height,
    0xff9e24ff,
    true,
  );
  console.timeEnd('edge::detect');
  ctx.putImageData(new ImageData(data, canvas.width, canvas.height), 0, 0);
  window.requestAnimationFrame(tick);
}

videoEl.addEventListener(
  'loadeddata',
  () => {
    console.log('video loaded');
    window.requestAnimationFrame(tick);
  },
  false,
);

(async () => {
  await wasm_bindgen(wasmURL);
  const video = {
    width: 480,
    height: 640,
    facingMode: 'environment',
  };

  videoEl.srcObject = await navigator.mediaDevices.getUserMedia({
    audio: false,
    video,
  });
})();
