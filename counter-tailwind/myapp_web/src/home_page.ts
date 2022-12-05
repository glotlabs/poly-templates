import init from "../wasm/myapp.js";
import { homePage } from "../wasm/myapp";
import { Poly } from "poly";
import { defaultDebugConfig } from "poly/src/logger";

(async () => {
  await init("/wasm/myapp_bg.wasm");

  const poly = new Poly(homePage(location.href), {
    loggerConfig: defaultDebugConfig(),
  });

  poly.init();
})();
