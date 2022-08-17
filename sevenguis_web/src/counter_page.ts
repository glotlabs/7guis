import init from "../wasm/sevenguis.js";
import { counterPage } from "../wasm/sevenguis";
import { Polyester } from "polyester";
import { defaultDebugConfig } from "polyester/src/logger";

(async () => {
  await init("./wasm/sevenguis_bg.wasm");

  const polyester = new Polyester(counterPage(), {
    loggerConfig: defaultDebugConfig(),
  });

  polyester.init();
})();
