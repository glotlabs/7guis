import init from "../wasm/sevenguis.js";
import { crudPage } from "../wasm/sevenguis";
import { Polyester } from "polyester";
import { defaultDebugConfig } from "polyester/src/logger";

(async () => {
  await init("/wasm/sevenguis_bg.wasm");

  const polyester = new Polyester(crudPage(), {
    loggerConfig: defaultDebugConfig(),
  });

  polyester.init();
})();
