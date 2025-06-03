import { Img, makeScene2D } from "@motion-canvas/2d";
import {
  all,
  easeOutCubic,
  ThreadGenerator,
  waitFor,
} from "@motion-canvas/core";
import { SceneLayout } from "ruspy-common/components";

import crabAss from "../../../assets/crab-ass.png";

export default makeScene2D(function* (view) {
  view.add(<SceneLayout />);
  SceneLayout.logo.remove();
  view.add(SceneLayout.logo);

  SceneLayout.logo.position(0);
  SceneLayout.logo.size(300);

  const crab = <Img x={1230} y={500} src={crabAss} /> as Img;
  view.add(crab);

  crab.moveDown();

  yield* waitFor(2);

  function* repeated(n: number, action: () => ThreadGenerator) {
    for (let _ = 0; _ < n; _++) {
      yield* action();
    }
  }

  yield* all(
    crab.position([13, 50], 10, easeOutCubic),
    crab.width(40, 10, easeOutCubic),
    repeated(20, function* () {
      yield* crab.rotation(-5, 0.25);
      yield* crab.rotation(5, 0.25);
    }),
  );

  yield* waitFor(2);
});
