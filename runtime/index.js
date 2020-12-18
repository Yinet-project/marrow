const fs = require("fs/promises");

const { print } = require("./debug");
const { _read_file_callback } = require("./fs");
const { _request_callback } = require("./request");

let actor_bin_instance = null;

let import_object = {
  wstd: {
    print,
    _read_file_callback,
    _request_callback,
  }
};

const run = async (modules) => {
  for (let item in modules) {
    try {
      const actor_bin_code = await fs.readFile(item.path);
      actor_bin_instance = (await WebAssembly.instantiate(actor_bin_code, import_object)).instance;
      actor_bin_instance.exports._entry();
    } catch (error) {
      console.log(error);
    }
  }
};

module.exports = {
  actor_bin_instance,
  run,
};
