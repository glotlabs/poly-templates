import init from "../wasm/myapp.js";
import { HomePage } from "../wasm/myapp";
import { Polyester } from "polyester";
import { defaultDebugConfig } from "polyester/src/logger";

(async () => {
  await init("./wasm/myapp_bg.wasm");

  const polyester = new Polyester(new HomePage(), {
    loggerConfig: defaultDebugConfig(),
  });

  polyester.init();
})();
