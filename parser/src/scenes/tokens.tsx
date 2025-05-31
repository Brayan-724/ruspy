import {
  Code,
  CodeRange,
  Img,
  makeScene2D,
  Rect,
  saturate,
  Txt,
  View2D,
  word,
} from "@motion-canvas/2d";
import {
  all,
  beginSlide,
  fadeTransition,
  sequence,
} from "@motion-canvas/core";
import { TreeNode } from "ruspy-common/components";
import { pyHighligther } from "ruspy-common/highlights";

import { EXAMPLE_CODE, EXAMPLE_CODE_2 } from "../commons";

import RustLangEsLogo from "../../../assets/Rust_Lang_ES_Logo.svg";
import CrabsBack from "../../../assets/crabs-back.png";

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
      code={EXAMPLE_CODE}
      position={[-500, -300]}
    />
  ) as Code;

  view.add(example_code);

  yield* fadeTransition(1);

  yield* all(
    example_code.code(EXAMPLE_CODE_2, 2),
    example_code.position([0, -400], 2),
    example_code.letterSpacing(10, 2),
  );

  yield* beginSlide("Show code");

  yield* sequence(
    0.5,
    token(view, example_code, -100, word(0, 0, 2), "Kw(If)"),
    token(view, example_code, 0, word(0, 3, 4), 'Literal("Hi")'),
    token(view, example_code, -100, word(0, 7, 1), "P(Colon)"),
    token(view, example_code, 0, word(0, 9, 6), 'Ident("output")'),
    token(view, example_code, -100, word(0, 16, 1), "P(Equal)"),
    token(view, example_code, 0, word(0, 18, 4), "Literal(True)"),
    token(view, example_code, -100, word(0, 23, 4), "Kw(Else)"),
    token(view, example_code, 0, word(0, 27, 1), "P(Colon)"),
    token(view, example_code, -100, word(0, 29, 6), 'Ident("output")'),
    token(view, example_code, 0, word(0, 36, 1), "P(Equal)"),
    token(view, example_code, -100, word(0, 38, 4), "Literal(True)"),
  );

  yield* beginSlide("End tokens");
});

function* token(
  view: View2D,
  code: Code,
  height: number,
  range: CodeRange,
  nodeName: string,
) {
  const [bbox] = code.getSelectionBBox(range);
  const x_mid = bbox.position.x + bbox.size.width / 2;

  const node = (
    <TreeNode
      position={[x_mid, height]}
      opacity={0}
      child={<Txt.b text={nodeName} fontSize={32} /> as Txt}
    />
  ) as TreeNode;

  const join = node.joinToPoint([x_mid, -380]);
  join.end(0);

  view.add(node);
  view.add(join);

  yield* sequence(
    0.25,
    join.end(1, 1),
    node.opacity(1, 1),
  );
}
