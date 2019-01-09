<meta charset="utf-8"/>

# Edge-Detection-Wasm
This is a simple rust library that is compiled down to WebAssembly using `wasm-pack`. 
The exported function (`detect`) takes in a pixel buffer in the form of a `ClampedUint8Array`
and outputs a new pixel buffer that has all the detected "edges" in that image highlighted 
with a given color

> Alot of this code is copy pasted from `https://github.com/PistonDevelopers/imageproc`

## 🚴 Usage

```js
import * as wasm from 'edge-detection-wasm';

const canvas = document.getElementById('canvas');
const ctx = canvas.getContext('2d');

const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
const data = wasm.detect(imageData.data, canvas.width, canvas.height, 0xFFFFFFFF);
ctx.putImageData(new ImageData(data, canvas.width, canvas.height), 0, 0);
```

## 🚴 Contributing

### 🛠️ Build with `wasm-pack build`

```
wasm-pack build
```

### 🛠️Start demo app with `npm start`

```
cd www && npm start
```

### 🎁 Publish to NPM with `wasm-pack publish`

```
wasm-pack publish
```

## 🏎 Speed
On my newish MBP, the `detect` function completes in about 29ms. I'd like to get it under 20ms.

## Size
- JavaScript < 700b (gzip)
- Wasm ~ 18kb (gzip)

