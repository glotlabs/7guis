import init, { temperaturePage } from "../wasm/sevenguis.js";
import { Polyester } from "polyester";
import { defaultDebugConfig } from "polyester/src/logger";

(async () => {
  await init("/wasm/sevenguis_bg.wasm");

  const polyester = new Polyester(temperaturePage(), {
    loggerConfig: defaultDebugConfig(),
  });

  polyester.init();
})();
