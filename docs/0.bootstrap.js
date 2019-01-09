(self["webpackJsonp"] = self["webpackJsonp"] || []).push([[0],{

/***/ "../pkg/edge_detection_wasm.js":
/*!*************************************!*\
  !*** ../pkg/edge_detection_wasm.js ***!
  \*************************************/
/*! exports provided: detect, __wbindgen_throw */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"detect\", function() { return detect; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_throw\", function() { return __wbindgen_throw; });\n/* harmony import */ var _edge_detection_wasm_bg__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./edge_detection_wasm_bg */ \"../pkg/edge_detection_wasm_bg.wasm\");\n/* tslint:disable */\n\n\nlet cachegetUint8Memory = null;\nfunction getUint8Memory() {\n    if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== _edge_detection_wasm_bg__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetUint8Memory = new Uint8Array(_edge_detection_wasm_bg__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetUint8Memory;\n}\n\nlet WASM_VECTOR_LEN = 0;\n\nfunction passArray8ToWasm(arg) {\n    const ptr = _edge_detection_wasm_bg__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"](arg.length * 1);\n    getUint8Memory().set(arg, ptr / 1);\n    WASM_VECTOR_LEN = arg.length;\n    return ptr;\n}\n\nlet cachegetUint8ClampedMemory = null;\nfunction getUint8ClampedMemory() {\n    if (cachegetUint8ClampedMemory === null || cachegetUint8ClampedMemory.buffer !== _edge_detection_wasm_bg__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetUint8ClampedMemory = new Uint8ClampedArray(_edge_detection_wasm_bg__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetUint8ClampedMemory;\n}\n\nfunction getClampedArrayU8FromWasm(ptr, len) {\n    return getUint8ClampedMemory().subarray(ptr / 1, ptr / 1 + len);\n}\n\nlet cachedGlobalArgumentPtr = null;\nfunction globalArgumentPtr() {\n    if (cachedGlobalArgumentPtr === null) {\n        cachedGlobalArgumentPtr = _edge_detection_wasm_bg__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_global_argument_ptr\"]();\n    }\n    return cachedGlobalArgumentPtr;\n}\n\nlet cachegetUint32Memory = null;\nfunction getUint32Memory() {\n    if (cachegetUint32Memory === null || cachegetUint32Memory.buffer !== _edge_detection_wasm_bg__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetUint32Memory = new Uint32Array(_edge_detection_wasm_bg__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetUint32Memory;\n}\n/**\n* @param {Uint8ClampedArray} arg0\n* @param {number} arg1\n* @param {number} arg2\n* @param {number} arg3\n* @returns {Uint8ClampedArray}\n*/\nfunction detect(arg0, arg1, arg2, arg3) {\n    const ptr0 = passArray8ToWasm(arg0);\n    const len0 = WASM_VECTOR_LEN;\n    const retptr = globalArgumentPtr();\n    _edge_detection_wasm_bg__WEBPACK_IMPORTED_MODULE_0__[\"detect\"](retptr, ptr0, len0, arg1, arg2, arg3);\n    const mem = getUint32Memory();\n    const rustptr = mem[retptr / 4];\n    const rustlen = mem[retptr / 4 + 1];\n\n    const realRet = getClampedArrayU8FromWasm(rustptr, rustlen).slice();\n    _edge_detection_wasm_bg__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_free\"](rustptr, rustlen * 1);\n    return realRet;\n\n}\n\nlet cachedTextDecoder = new TextDecoder('utf-8');\n\nfunction getStringFromWasm(ptr, len) {\n    return cachedTextDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));\n}\n\nfunction __wbindgen_throw(ptr, len) {\n    throw new Error(getStringFromWasm(ptr, len));\n}\n\n\n\n//# sourceURL=webpack:///../pkg/edge_detection_wasm.js?");

/***/ }),

/***/ "../pkg/edge_detection_wasm_bg.wasm":
/*!******************************************!*\
  !*** ../pkg/edge_detection_wasm_bg.wasm ***!
  \******************************************/
/*! exports provided: memory, detect, __wbindgen_global_argument_ptr, __wbindgen_malloc, __wbindgen_free */
/***/ (function(module, exports, __webpack_require__) {

eval("\"use strict\";\n// Instantiate WebAssembly module\nvar wasmExports = __webpack_require__.w[module.i];\n__webpack_require__.r(exports);\n// export exports from WebAssembly module\nfor(var name in wasmExports) if(name != \"__webpack_init__\") exports[name] = wasmExports[name];\n// exec imports from WebAssembly module (for esm order)\n/* harmony import */ var m0 = __webpack_require__(/*! ./edge_detection_wasm */ \"../pkg/edge_detection_wasm.js\");\n\n\n// exec wasm module\nwasmExports[\"__webpack_init__\"]()\n\n//# sourceURL=webpack:///../pkg/edge_detection_wasm_bg.wasm?");

/***/ }),

/***/ "./index.js":
/*!******************!*\
  !*** ./index.js ***!
  \******************/
/*! no exports provided */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var edge_detection_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! edge-detection-wasm */ \"../pkg/edge_detection_wasm.js\");\n// import Worker from 'worker-loader!./worker.js';\n\n\nconst canvas = document.getElementById('canvas');\nconst videoEl = document.getElementById('video');\nconst ctx = canvas.getContext('2d');\nlet count = 0;\n\nfunction tick() {\n  ctx.drawImage(videoEl, 0, 0, canvas.width, canvas.height);\n  const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);\n  console.time('edge::detect');\n  const data = edge_detection_wasm__WEBPACK_IMPORTED_MODULE_0__[\"detect\"](\n    imageData.data,\n    canvas.width,\n    canvas.height,\n    0xff9e24ff,\n  );\n  console.timeEnd('edge::detect');\n  ctx.putImageData(new ImageData(data, canvas.width, canvas.height), 0, 0);\n  count++;\n  window.requestAnimationFrame(tick);\n}\n\nvideoEl.addEventListener(\n  'loadeddata',\n  () => {\n    console.log('video loaded');\n    window.requestAnimationFrame(tick);\n  },\n  false,\n);\n\nconst video = {\n  width: 640,\n  height: 480,\n  facingMode: 'environment',\n};\n\nnavigator.mediaDevices\n  .getUserMedia({\n    audio: false,\n    video,\n  })\n  .then(stream => {\n    videoEl.srcObject = stream;\n  })\n  .catch(err => {\n    console.error('media device err', err);\n  });\n\n\n//# sourceURL=webpack:///./index.js?");

/***/ })

}]);