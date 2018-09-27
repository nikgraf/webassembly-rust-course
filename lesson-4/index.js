const utils = import("./pkg/utils");

utils.then(wasmModule => {
  wasmModule.run();
});
