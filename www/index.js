// import Worker from 'worker-loader!./worker.js';
import * as wasm from 'edge-detection-wasm';
console.log(wasm);

const canvas = document.getElementById('canvas');
const videoEl = document.getElementById('video');
const ctx = canvas.getContext('2d');
let count = 0;

function tick() {
  ctx.drawImage(videoEl, 0, 0, canvas.width, canvas.height);
  const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
  console.time('edge::detect');
  const data = wasm.detect(imageData.data, canvas.width, canvas.height, count);
  console.timeEnd('edge::detect');
  ctx.putImageData(new ImageData(data, canvas.width, canvas.height), 0, 0);
  count++;
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

const video = {
  width: 640,
  height: 480,
  facingMode: 'environment',
};

navigator.mediaDevices
  .getUserMedia({
    audio: false,
    video,
  })
  .then(stream => {
    videoEl.srcObject = stream;
  })
  .catch(err => {
    console.error('media device err', err);
  });
