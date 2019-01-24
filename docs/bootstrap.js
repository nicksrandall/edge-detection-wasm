/******/ (function(modules) { // webpackBootstrap
/******/ 	// The module cache
/******/ 	var installedModules = {};
/******/
/******/ 	// The require function
/******/ 	function __webpack_require__(moduleId) {
/******/
/******/ 		// Check if module is in cache
/******/ 		if(installedModules[moduleId]) {
/******/ 			return installedModules[moduleId].exports;
/******/ 		}
/******/ 		// Create a new module (and put it into the cache)
/******/ 		var module = installedModules[moduleId] = {
/******/ 			i: moduleId,
/******/ 			l: false,
/******/ 			exports: {}
/******/ 		};
/******/
/******/ 		// Execute the module function
/******/ 		modules[moduleId].call(module.exports, module, module.exports, __webpack_require__);
/******/
/******/ 		// Flag the module as loaded
/******/ 		module.l = true;
/******/
/******/ 		// Return the exports of the module
/******/ 		return module.exports;
/******/ 	}
/******/
/******/
/******/ 	// expose the modules object (__webpack_modules__)
/******/ 	__webpack_require__.m = modules;
/******/
/******/ 	// expose the module cache
/******/ 	__webpack_require__.c = installedModules;
/******/
/******/ 	// define getter function for harmony exports
/******/ 	__webpack_require__.d = function(exports, name, getter) {
/******/ 		if(!__webpack_require__.o(exports, name)) {
/******/ 			Object.defineProperty(exports, name, { enumerable: true, get: getter });
/******/ 		}
/******/ 	};
/******/
/******/ 	// define __esModule on exports
/******/ 	__webpack_require__.r = function(exports) {
/******/ 		if(typeof Symbol !== 'undefined' && Symbol.toStringTag) {
/******/ 			Object.defineProperty(exports, Symbol.toStringTag, { value: 'Module' });
/******/ 		}
/******/ 		Object.defineProperty(exports, '__esModule', { value: true });
/******/ 	};
/******/
/******/ 	// create a fake namespace object
/******/ 	// mode & 1: value is a module id, require it
/******/ 	// mode & 2: merge all properties of value into the ns
/******/ 	// mode & 4: return value when already ns object
/******/ 	// mode & 8|1: behave like require
/******/ 	__webpack_require__.t = function(value, mode) {
/******/ 		if(mode & 1) value = __webpack_require__(value);
/******/ 		if(mode & 8) return value;
/******/ 		if((mode & 4) && typeof value === 'object' && value && value.__esModule) return value;
/******/ 		var ns = Object.create(null);
/******/ 		__webpack_require__.r(ns);
/******/ 		Object.defineProperty(ns, 'default', { enumerable: true, value: value });
/******/ 		if(mode & 2 && typeof value != 'string') for(var key in value) __webpack_require__.d(ns, key, function(key) { return value[key]; }.bind(null, key));
/******/ 		return ns;
/******/ 	};
/******/
/******/ 	// getDefaultExport function for compatibility with non-harmony modules
/******/ 	__webpack_require__.n = function(module) {
/******/ 		var getter = module && module.__esModule ?
/******/ 			function getDefault() { return module['default']; } :
/******/ 			function getModuleExports() { return module; };
/******/ 		__webpack_require__.d(getter, 'a', getter);
/******/ 		return getter;
/******/ 	};
/******/
/******/ 	// Object.prototype.hasOwnProperty.call
/******/ 	__webpack_require__.o = function(object, property) { return Object.prototype.hasOwnProperty.call(object, property); };
/******/
/******/ 	// __webpack_public_path__
/******/ 	__webpack_require__.p = "";
/******/
/******/
/******/ 	// Load entry module and return exports
/******/ 	return __webpack_require__(__webpack_require__.s = "./bootstrap.js");
/******/ })
/************************************************************************/
/******/ ({

/***/ "../pkg/edge_detection_wasm.js":
/*!*************************************!*\
  !*** ../pkg/edge_detection_wasm.js ***!
  \*************************************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("(function() {\n    var wasm;\n    const __exports = {};\n\n\n    let cachegetUint8Memory = null;\n    function getUint8Memory() {\n        if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== wasm.memory.buffer) {\n            cachegetUint8Memory = new Uint8Array(wasm.memory.buffer);\n        }\n        return cachegetUint8Memory;\n    }\n\n    let WASM_VECTOR_LEN = 0;\n\n    function passArray8ToWasm(arg) {\n        const ptr = wasm.__wbindgen_malloc(arg.length * 1);\n        getUint8Memory().set(arg, ptr / 1);\n        WASM_VECTOR_LEN = arg.length;\n        return ptr;\n    }\n\n    let cachegetUint8ClampedMemory = null;\n    function getUint8ClampedMemory() {\n        if (cachegetUint8ClampedMemory === null || cachegetUint8ClampedMemory.buffer !== wasm.memory.buffer) {\n            cachegetUint8ClampedMemory = new Uint8ClampedArray(wasm.memory.buffer);\n        }\n        return cachegetUint8ClampedMemory;\n    }\n\n    function getClampedArrayU8FromWasm(ptr, len) {\n        return getUint8ClampedMemory().subarray(ptr / 1, ptr / 1 + len);\n    }\n\n    let cachedGlobalArgumentPtr = null;\n    function globalArgumentPtr() {\n        if (cachedGlobalArgumentPtr === null) {\n            cachedGlobalArgumentPtr = wasm.__wbindgen_global_argument_ptr();\n        }\n        return cachedGlobalArgumentPtr;\n    }\n\n    let cachegetUint32Memory = null;\n    function getUint32Memory() {\n        if (cachegetUint32Memory === null || cachegetUint32Memory.buffer !== wasm.memory.buffer) {\n            cachegetUint32Memory = new Uint32Array(wasm.memory.buffer);\n        }\n        return cachegetUint32Memory;\n    }\n    /**\n    * @param {Uint8Array} arg0\n    * @param {number} arg1\n    * @param {number} arg2\n    * @param {number} arg3\n    * @param {boolean} arg4\n    * @returns {Uint8ClampedArray}\n    */\n    __exports.detect = function(arg0, arg1, arg2, arg3, arg4) {\n        const ptr0 = passArray8ToWasm(arg0);\n        const len0 = WASM_VECTOR_LEN;\n        const retptr = globalArgumentPtr();\n        wasm.detect(retptr, ptr0, len0, arg1, arg2, arg3, arg4);\n        const mem = getUint32Memory();\n        const rustptr = mem[retptr / 4];\n        const rustlen = mem[retptr / 4 + 1];\n\n        const realRet = getClampedArrayU8FromWasm(rustptr, rustlen).slice();\n        wasm.__wbindgen_free(rustptr, rustlen * 1);\n        return realRet;\n\n    };\n\n    let cachedTextDecoder = new TextDecoder('utf-8');\n\n    function getStringFromWasm(ptr, len) {\n        return cachedTextDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));\n    }\n\n    __exports.__wbindgen_throw = function(ptr, len) {\n        throw new Error(getStringFromWasm(ptr, len));\n    };\n\n    function init(path_or_module) {\n        let instantiation;\n        const imports = { './edge_detection_wasm': __exports };\n        if (path_or_module instanceof WebAssembly.Module) {\n            instantiation = WebAssembly.instantiate(path_or_module, imports)\n            .then(instance => {\n            return { instance, module: path_or_module }\n        });\n    } else {\n        const data = fetch(path_or_module);\n        if (typeof WebAssembly.instantiateStreaming === 'function') {\n            instantiation = WebAssembly.instantiateStreaming(data, imports);\n        } else {\n            instantiation = data\n            .then(response => response.arrayBuffer())\n            .then(buffer => WebAssembly.instantiate(buffer, imports));\n        }\n    }\n    return instantiation.then(({instance}) => {\n        wasm = init.wasm = instance.exports;\n\n    });\n};\nself.wasm_bindgen = Object.assign(init, __exports);\n})();\n\n\n//# sourceURL=webpack:///../pkg/edge_detection_wasm.js?");

/***/ }),

/***/ "../pkg/edge_detection_wasm_bg.wasm":
/*!******************************************!*\
  !*** ../pkg/edge_detection_wasm_bg.wasm ***!
  \******************************************/
/*! no static exports found */
/***/ (function(module, exports, __webpack_require__) {

eval("module.exports = __webpack_require__.p + \"edge_detection_wasm_bg.acccc.wasm\";\n\n//# sourceURL=webpack:///../pkg/edge_detection_wasm_bg.wasm?");

/***/ }),

/***/ "./bootstrap.js":
/*!**********************!*\
  !*** ./bootstrap.js ***!
  \**********************/
/*! no exports provided */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var edge_detection_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! edge-detection-wasm */ \"../pkg/edge_detection_wasm.js\");\n/* harmony import */ var edge_detection_wasm__WEBPACK_IMPORTED_MODULE_0___default = /*#__PURE__*/__webpack_require__.n(edge_detection_wasm__WEBPACK_IMPORTED_MODULE_0__);\n/* harmony import */ var edge_detection_wasm_edge_detection_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! edge-detection-wasm/edge_detection_wasm_bg.wasm */ \"../pkg/edge_detection_wasm_bg.wasm\");\n/* harmony import */ var edge_detection_wasm_edge_detection_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_1___default = /*#__PURE__*/__webpack_require__.n(edge_detection_wasm_edge_detection_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_1__);\n\n\n\nlet wasm;\nconst canvas = document.getElementById('canvas');\nconst videoEl = document.getElementById('video');\nconst ctx = canvas.getContext('2d');\n\nfunction tick() {\n  ctx.drawImage(videoEl, 0, 0, canvas.width, canvas.height);\n  const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);\n  console.time('edge::detect');\n  const data = wasm_bindgen.detect(\n    imageData.data,\n    canvas.width,\n    canvas.height,\n    0xff9e24ff,\n    false,\n  );\n  console.timeEnd('edge::detect');\n  ctx.putImageData(new ImageData(data, canvas.width, canvas.height), 0, 0);\n  window.requestAnimationFrame(tick);\n}\n\nvideoEl.addEventListener(\n  'loadeddata',\n  () => {\n    console.log('video loaded');\n    window.requestAnimationFrame(tick);\n  },\n  false,\n);\n\n(async () => {\n  await wasm_bindgen(edge_detection_wasm_edge_detection_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_1___default.a);\n  const video = {\n    width: 480,\n    height: 640,\n    facingMode: 'environment',\n  };\n\n  videoEl.srcObject = await navigator.mediaDevices.getUserMedia({\n    audio: false,\n    video,\n  });\n})();\n\n\n//# sourceURL=webpack:///./bootstrap.js?");

/***/ })

/******/ });