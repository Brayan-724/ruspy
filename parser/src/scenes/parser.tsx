import { makeScene2D, Txt } from "@motion-canvas/2d";
import { all, waitFor } from "@motion-canvas/core";

import {
  Chapters,
  Code,
  CodeTokenizer,
  SceneLayout,
  Token,
  TreeGraph,
  TreeNode,
} from "ruspy-common/components";

import { EXAMPLE_CODE_2, EXAMPLE_CODE_TOKENS } from "../commons";

export default makeScene2D(function* (view) {
  let example_code: Code;
  let tokens: Token[];
  yield* SceneLayout.setup(view, function* () {
    yield* Chapters.spotOne([1], 0);

    example_code = (
      <Code
        code={EXAMPLE_CODE_2}
        letterSpacing={1}
      />
    ) as Code;

    example_code.cursorRect.opacity(0);
    example_code.fill("#0000");

    view.add(example_code);

    const tokenizer = new CodeTokenizer(example_code);
    tokens = tokenizer.tokenize(EXAMPLE_CODE_TOKENS);
    example_code.boxSelection([0, 0]);
  });

  yield* example_code.y(-400, 1);

  const graph = (
    <TreeGraph
      x={0}
      y={100}
      nodes={[
        tokens[0].container.clone({
          x: 0,
          y: -270,
          offsetX: 0,
        }),

        tokens[1].container.clone({ x: -400, y: 0, offsetX: 0 }),

        <TreeNode
          x={0}
          child={<Txt.b fontFamily="Inter" text="VarDecl" /> as Txt}
        />,
        <TreeNode
          x={400}
          child={<Txt.b fontFamily="Inter" text="VarDecl" /> as Txt}
        />,

        tokens[3].container.clone({ x: -200, y: 270, offsetX: 0 }),
        tokens[5].container.clone({ x: +250, y: 270, offsetX: 0 }),
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

  const tokenizer = new CodeTokenizer(example_code);

  const retreiveToken = function *(graphNode: number, token: number) {
    const clone = graph.nodes()[graphNode].snapshotClone({
      opacity: 1,
      offsetX: -1,
    });

    view.add(clone);

    clone.absolutePosition(tokens[token].container.absolutePosition);

    yield* all(
      clone.absolutePosition(graph.nodes()[graphNode].absolutePosition, 1),
      clone.offset.x(0, 1),
    );

    clone.remove()
  };

  yield* all(
    retreiveToken(0, 0),
    graph.nodes()[0].opacity(1, 1),
    graph.highlightNode(0, 1),
    example_code.cursorRect.opacity(1, 0.25),
    tokenizer.nextToken(1, [0, 2], false),
  );

  yield* all(
    retreiveToken(1, 1),
    graph.showJoin(0, 1),
    graph.highlightJoin(0, 1),
    tokenizer.nextToken(1, [1, 4], false),
  );

  yield* all(
    graph.showJoin(1, 1),
    graph.highlightJoin(1, 1),
    tokenizer.nextToken(1, [2, 13], false),
  );

  yield* all(
    retreiveToken(4, 3),
    graph.showJoin(3, 1),
    graph.highlightJoin(3, 1),
    tokenizer.nextToken(1, [-13, 6], false),
  );

  yield* all(
    retreiveToken(5, 5),
    graph.showJoin(4, 1),
    graph.highlightJoin(4, 1),
    tokenizer.nextToken(1, [3, 4], false),
  );

  yield* tokenizer.nextToken(1, [1, 4], false);

  yield* all(
    graph.showJoin(2, 1),
    graph.highlightJoin(2, 1),
    tokenizer.nextToken(1, [2, 14], false),
  );

  yield* all(
    graph.unhighlight(1),
    tokenizer.nextToken(1, [0, 0], false),
    example_code.cursorRect.opacity(0, 0.75),
  );

  yield* waitFor(1);
});
