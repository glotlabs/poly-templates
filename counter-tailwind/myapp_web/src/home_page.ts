import init from "../wasm/myapp.js";
import { homePage } from "../wasm/myapp";
import { Polyester } from "polyester";
import { defaultDebugConfig } from "polyester/src/logger";

(async () => {
  await init("./wasm/myapp_bg.wasm");

  const polyester = new Polyester(homePage(), {
    loggerConfig: defaultDebugConfig(),
  });

  polyester.init();
})();
