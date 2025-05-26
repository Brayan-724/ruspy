import {
  Code,
  Img,
  lines,
  makeScene2D,
  Rect,
  saturate,
  Txt,
  word,
} from "@motion-canvas/2d";
import { all, beginSlide, DEFAULT, fadeTransition, waitFor } from "@motion-canvas/core";
import { TreeGraph, TreeNode } from "ruspy-common/components";
import { pyHighligther } from "ruspy-common/highlights";

import RustLangEsLogo from "../../../assets/Rust_Lang_ES_Logo.svg";
import CrabsBack from "../../../assets/crabs-back.png";

import "../fonts.css";

const EXAMPLE_CODE = `\
if "Hi":
  output = True
else:
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

  yield* fadeTransition(1);

  const example_code = (
    <Code highlighter={pyHighligther} code={EXAMPLE_CODE} scale={0} />
  ) as Code;

  view.add(example_code);

  yield* example_code.scale(1, 1);

  yield* beginSlide("SHOW CODE");

  const graph = (
    <TreeGraph
      x={200}
      y={100}
      nodes={[
        <TreeNode
          y={-270}
          child={<Txt.b fontFamily="Inter" text="Conditional" /> as Txt}
        />,

        <TreeNode
          x={-400}
          child={<Txt.b fontFamily="Inter" text='Literal("Hi")' /> as Txt}
        />,
        <TreeNode
          x={0}
          child={<Txt.b fontFamily="Inter" text="VarDecl" /> as Txt}
        />,
        <TreeNode
          x={400}
          child={<Txt.b fontFamily="Inter" text="VarDecl" /> as Txt}
        />,

        <TreeNode
          y={270}
          x={-200}
          child={<Txt.b fontFamily="Inter" text="output" /> as Txt}
        />,
        <TreeNode
          y={270}
          x={250}
          child={<Txt.b fontFamily="Inter" text="Literal(True)" /> as Txt}
        />,
      ]}
      joins={[
        [0, 1, "test"],
        [0, 2, "body"],
        [0, 3, "otherwise"],

        [2, 4, "var"],
        [2, 5, "expr"],
      ]}
    />
  ) as TreeGraph;

  view.add(graph);

  for (const node of graph.joinNodes()) {
    node[0].opacity(0);
    node[1].opacity(0);
    node[2].end(0);
    node[3].opacity(0);
  }

  yield* all(
    example_code.position([-500, -300], 1),
  );

  yield* all(
    graph.nodes()[0].opacity(1, 1),
    graph.highlightNode(0, 1),
    example_code.selection(word(0, 0, 2), 1),
  );

  yield* beginSlide("INSPECT [if]");

  yield* all(
    graph.showJoin(0, 1),
    example_code.selection(word(0, 3, 4), 1),
    graph.highlightJoin(0, 1),
  );

  yield* beginSlide("INSPECT [Hi]");

  yield* all(
    graph.showJoin(1, 1),
    graph.highlightJoin(1, 1),
    example_code.selection(lines(1), 1),
  );

  yield* beginSlide("INSPECT [body]");

  yield* all(
    graph.showJoin(2, 1),
    graph.highlightJoin(2, 1),
    example_code.selection(lines(3), 1),
  );

  yield* beginSlide("INSPECT [otherwise]");

  yield* all(
    graph.showJoin(3, 1),
    graph.highlightJoin(3, 1),
    example_code.selection(word(1, 2, "output".length), 1),
  );

  yield* beginSlide("INSPECT [output]");

  yield* all(
    graph.showJoin(4, 1),
    graph.highlightJoin(4, 1),
    example_code.selection(word(1, 11, "True".length), 1),
  );

  yield* beginSlide("INSPECT [true]");

  yield* all(
    graph.unhighlight(1),
    example_code.selection(DEFAULT, 1),
  );

  yield* waitFor(1);
});
