import { Circle, Img, makeScene2D, Rect } from "@motion-canvas/2d";
import {
  all,
  beginSlide,
  easeInCirc,
  easeInQuad,
  easeOutBounce,
  sequence,
} from "@motion-canvas/core";

import RustLangEsLogo from "../../../assets/Rust_Lang_ES_Logo.svg";

export default makeScene2D(function* (view) {
  yield view.add(<Rect width={1920} height={1080} fill="#2E2E2E" />);

  const logo = <Img src={RustLangEsLogo} width={0} /> as Img;
  const wave_out = <Circle width={0} height={0} fill="#9A8AFB" /> as Circle;
  const wave_in = <Circle width={0} height={0} fill="#2E2E2E" /> as Circle;

  view.add(wave_out);
  view.add(wave_in);
  view.add(logo);

  yield logo;

  yield* sequence(
    0.4,
    logo.width(300, 2, easeOutBounce),
    all(
      wave_out.width(2500, 2),
      wave_out.height(2500, 2),
      wave_in.width(2220, 2),
      wave_in.height(2220, 2),
    ),
  );

  yield* beginSlide("INTRO");

  yield* all(
    logo.position([-850, 450], 2),
    logo.width(150, 2),
  );
});
