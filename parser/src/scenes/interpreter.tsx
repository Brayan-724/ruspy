import { makeScene2D, Node } from "@motion-canvas/2d";
import {
  Chapters,
  Code,
  CodeColors,
  CodeTokenizer,
  SceneLayout,
  TokenBox,
  TreeGraph,
} from "ruspy-common/components";
import { EXAMPLE_CODE_2, EXAMPLE_CODE_TOKENS } from "../commons";
import { all, createSignal, easeOutBounce, waitFor } from "@motion-canvas/core";

export default makeScene2D(function* (view) {
  let example_code: Code;
  let graph: TreeGraph;
  yield* SceneLayout.setup(view, function* () {
    yield* Chapters.spotOne([2], 0);

    example_code = (
      <Code
        code={EXAMPLE_CODE_2}
        letterSpacing={1}
        fill="#0000"
      />
    ) as Code;

    example_code.cursorRect.opacity(0);

    const tokenizer = new CodeTokenizer(example_code);
    const tokens = tokenizer.tokenize(EXAMPLE_CODE_TOKENS);

    graph = (
      <TreeGraph
        y={100}
        nodes={[
          tokens[0].container.clone({
            x: 0,
            y: -270,
            offsetX: 0,
          }),

          tokens[1].container.clone({ x: -400, y: 0, offsetX: 0 }),

          tokens[4].container.clone({ x: 0, offsetX: 0 }),
          tokens[3].container.clone({ x: -100, y: 270, offsetX: 0 }),
          tokens[5].container.clone({ x: +100, y: 270, offsetX: 0 }),

          tokens[4].container.clone({ x: 400, offsetX: 0 }),
          tokens[8].container.clone({ x: 300, y: 270, offsetX: 0 }),
          tokens[10].container.clone({ x: +500, y: 270, offsetX: 0 }),

          <TokenBox color={CodeColors.ORANGE} text="True" x={-400} y={270} />,
        ]}
        joins={[
          [0, 1, "test"],

          [0, 2, "body"],
          [2, 3, "var"],
          [2, 4, "expr"],

          [0, 5, "otherwise"],
          [5, 6, "var"],
          [5, 7, "expr"],

          [1, 8, ""],
        ]}
      />
    ) as TreeGraph;

    {
      const [, to, curve] = graph.joinNodes()[7];
      to.opacity(0);
      curve.end(0);
    }

    view.add(graph);
  });

  const outputValue = createSignal("nil");
  const outputNode = (
    <Node y={-400} opacity={0}>
      <TokenBox
        x={-50}
        color={CodeColors.CYAN}
        text="output"
        offsetX={1}
        textProps={{ letterSpacing: 1, fontFamily: "monospace" }}
      />
      <TokenBox
        color={CodeColors.GREEN}
        text="="
        textProps={{ letterSpacing: 1, fontFamily: "monospace" }}
      />
      <TokenBox
        x={50}
        color={CodeColors.ORANGE}
        text={outputValue}
        offsetX={-1}
        textProps={{ letterSpacing: 1, fontFamily: "monospace" }}
      />
    </Node>
  );

  view.add(outputNode);

  /// SHOW OUTPUT
  yield* outputNode.opacity(1, 1);

  yield* waitFor(1);

  yield* outputNode.opacity(0.25, 1);

  /// GO TEST
  yield* graph.highlightJoin(0, 1);

  yield* all(
    graph.showJoin(7, 1),
    graph.highlightJoins([0, 7], 1),
  );

  yield* graph.highlightNode(8, 1);

  yield* waitFor(1);

  /// GO BODY
  yield* graph.highlightJoins([1], 0.5);
  yield* graph.highlightJoins([1, 2, 3], 0.5);

  yield* waitFor(1);
  yield* outputNode.opacity(1, 0.3);
  yield* outputValue("True", 1);

  yield* waitFor(1);
  yield* outputNode.opacity(0.25, 0.3);

  /// UPDATE TEST
  yield* graph.highlightNode(1, 1);
  yield* (graph.nodes()[1] as TokenBox).textNode.text('""', 1);
  yield* graph.highlightNode(8, 1);
  yield* (graph.nodes()[8] as TokenBox).text("False", 1);

  yield* waitFor(1);

  /// GO OTHERWISE
  yield* graph.highlightJoins([4], 0.5);
  yield* graph.highlightJoins([4, 5, 6], 0.5);

  yield* waitFor(1);

  yield* outputNode.opacity(1, 0.3);
  yield* outputValue("False", 1);

  yield* waitFor(1);

  yield* all(
    graph.opacity(0, 1),
    outputNode.opacity(0, 1),
  );

  yield* all(
    Chapters.slideOut(1),
    SceneLayout.logo.position(0, 1),
    SceneLayout.logo.size(300, 1),
  );
});
