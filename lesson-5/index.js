const utils = import("./utils");

utils.then(wasmModule => {
  wasmModule.run();
});
