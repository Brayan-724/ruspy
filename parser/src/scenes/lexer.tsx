import { Code, Img, makeScene2D, Rect, saturate } from "@motion-canvas/2d";
import { all, fadeTransition, waitFor } from "@motion-canvas/core";
import { pyHighligther } from "ruspy-common/highlights";

import RustLangEsLogo from "../../../assets/Rust_Lang_ES_Logo.svg";
import CrabsBack from "../../../assets/crabs-back.png";

const EXAMPLE_CODE_2 = `\
if "Hi":\
 output = True \
else:\
 output = False\
`;

export default makeScene2D(function* (view) {
  view.add(<Rect width={1920} height={1080} fill="#2E2E2E" />);

  view.add(
    <Img
      src={CrabsBack}
      width={1920}
      height={1080}
      opacity={0.05}
      filters={[saturate(0.3)]}
    />,
  );
  view.add(<Img src={RustLangEsLogo} x={-850} y={450} width={150} />);

  const example_code = (
    <Code
      highlighter={pyHighligther}
      letterSpacing={10}
      code={EXAMPLE_CODE_2}
      position={[-0, -400]}
    />
  ) as Code;

  view.add(example_code);

  yield* fadeTransition(1);

  const code_w = example_code.width() / 2;

  const cursor = (
    <Rect
      x={-130}
      y={-300}
      width={0}
      stroke="#FFF"
      lineWidth={10}
      radius={12}
      height={example_code.height}
    />
  );

  view.add(cursor);

  yield* all(
    example_code.fontSize(60, 2),
    example_code.position([code_w, -300], 2),
  );

  yield* waitFor(1);
});
