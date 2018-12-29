import * as wasm from 'edge-detection-wasm';

console.log('wasm', wasm);

export default function go(event) {
  if (!wasm) {
    self.postMessage(event.data, [event.data.data.buffer]);
  } else {
    const start = performance.now();
    const width = event.data.width;
    const height = event.data.height;
    const data = wasm.detect(event.data.data, width, height);
    self.postMessage({data, width, height}, [data.buffer]);
    const end = performance.now();
    console.log(`detect took ${end - start}ms`);
  }
}
