<meta charset="utf-8"/>

# Edge-Detection-Wasm
This is a simple rust library that is compiled down to WebAssembly using `wasm-pack`. 
The exported function (`detect`) takes in a pixel buffer in the form of a `ClampedUint8Array`
and outputs a new pixel buffer that has all the detected "edges" in that image highlighted 
with a random color (follows cubehelix color space for effect).

> Alot of this code is copy pasted from `https://github.com/PistonDevelopers/imageproc`

## 🚴 Usage

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
On my newish MBP, the `detect` function completes in about 31ms. I'd like to get it under 20ms.

## Size
- JavaScript < 700b (gzip)
- Wasm ~ 18kb (gzip)

