import init, { flightPage } from "../wasm/sevenguis.js";
import { Polyester } from "polyester";
import { posixFromMilliseconds } from "polyester/src/time";

(async () => {
  await init("/wasm/sevenguis_bg.wasm");

  const currentTime = posixFromMilliseconds(Date.now());

  const polyester = new Polyester(flightPage(currentTime));
  polyester.init();
})();
