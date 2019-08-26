(window["webpackJsonp"] = window["webpackJsonp"] || []).push([[0],{

/***/ "../pkg/wasm_audio.js":
/*!****************************!*\
  !*** ../pkg/wasm_audio.js ***!
  \****************************/
/*! exports provided: greet, __wbg_alert_708c5a322eb72756 */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"greet\", function() { return greet; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_alert_708c5a322eb72756\", function() { return __wbg_alert_708c5a322eb72756; });\n/* harmony import */ var _wasm_audio_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./wasm_audio_bg.wasm */ \"../pkg/wasm_audio_bg.wasm\");\n\n\n/**\n*/\nfunction greet() {\n    _wasm_audio_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"greet\"]();\n}\n\nlet cachedTextDecoder = new TextDecoder('utf-8');\n\nlet cachegetUint8Memory = null;\nfunction getUint8Memory() {\n    if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== _wasm_audio_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetUint8Memory = new Uint8Array(_wasm_audio_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetUint8Memory;\n}\n\nfunction getStringFromWasm(ptr, len) {\n    return cachedTextDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));\n}\n\nconst __wbg_alert_708c5a322eb72756 = function(arg0, arg1) {\n    alert(getStringFromWasm(arg0, arg1));\n};\n\n\n\n//# sourceURL=webpack:///../pkg/wasm_audio.js?");

/***/ }),

/***/ "../pkg/wasm_audio_bg.wasm":
/*!*********************************!*\
  !*** ../pkg/wasm_audio_bg.wasm ***!
  \*********************************/
/*! exports provided: memory, greet */
/***/ (function(module, exports, __webpack_require__) {

eval("\"use strict\";\n// Instantiate WebAssembly module\nvar wasmExports = __webpack_require__.w[module.i];\n__webpack_require__.r(exports);\n// export exports from WebAssembly module\nfor(var name in wasmExports) if(name != \"__webpack_init__\") exports[name] = wasmExports[name];\n// exec imports from WebAssembly module (for esm order)\n/* harmony import */ var m0 = __webpack_require__(/*! ./wasm_audio.js */ \"../pkg/wasm_audio.js\");\n\n\n// exec wasm module\nwasmExports[\"__webpack_init__\"]()\n\n//# sourceURL=webpack:///../pkg/wasm_audio_bg.wasm?");

/***/ }),

/***/ "./index.js":
/*!******************!*\
  !*** ./index.js ***!
  \******************/
/*! no exports provided */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var wasm_audio__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! wasm-audio */ \"../pkg/wasm_audio.js\");\n\n\nwasm_audio__WEBPACK_IMPORTED_MODULE_0__[\"greet\"]();\n\n\n//# sourceURL=webpack:///./index.js?");

/***/ })

}]);