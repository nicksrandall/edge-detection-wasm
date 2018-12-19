const fs = require('fs');
const binaryen = require('binaryen');

const myModule = binaryen.readBinary(
  fs.readFileSync('./pkg/edge_detection_wasm_bg.wasm', null),
);

binaryen.setOptimizeLevel(2);
binaryen.setShrinkLevel(1);
myModule.optimize();

fs.writeFileSync(
  './pkg/edge_detection_wasm_bg_opt.wasm',
  myModule.emitBinary(),
);
