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
        ]}
        joins={[
          [0, 1, "test"],

          [0, 2, "body"],
          [2, 3, "var"],
          [2, 4, "expr"],

          [0, 5, "otherwise"],
          [5, 6, "var"],
          [5, 7, "expr"],
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

  const retreiveToken = function *(graphNode: number, token: number, duration = 1) {
    const clone = graph.nodes()[graphNode].snapshotClone({
      opacity: 1,
      offsetX: -1,
    });

    view.add(clone);

    clone.absolutePosition(tokens[token].container.absolutePosition);

    yield* all(
      clone.absolutePosition(graph.nodes()[graphNode].absolutePosition, duration),
      clone.offset.x(0, duration),
    );

    clone.remove()
  };

  // [if]
  yield* all(
    retreiveToken(0, 0),
    graph.nodes()[0].opacity(1, 1),
    graph.highlightNode(0, 1),
    example_code.cursorRect.opacity(1, 0.25),
    tokenizer.nextToken(1, [0, 2], false),
  );

  // ["Hello"]
  yield* all(
    retreiveToken(1, 1),
    graph.showJoin(0, 1),
    graph.highlightJoin(0, 1),
    tokenizer.nextToken(1, [1, 4], false),
  );

  // VarDecl
  yield* all(
    graph.showJoin(1, 1),
    graph.highlightJoin(1, 1),
    tokenizer.nextToken(1, [2, 13], false),
  );

  // [output]
  yield* all(
    retreiveToken(3, 3),
    graph.showJoin(2, 1),
    graph.highlightJoin(2, 1),
    tokenizer.nextToken(1, [-13, 6], false),
  );

  // [True]
  yield* all(
    retreiveToken(4, 5),
    graph.showJoin(3, 1),
    graph.highlightJoin(3, 1),
    tokenizer.nextToken(1, [3, 4], false),
  );

  // [else]
  yield* tokenizer.nextToken(1, [1, 4], false);

  // VarDecl
  yield* all(
    graph.showJoin(4, 0.5),
    graph.highlightJoin(4, 0.5),
    tokenizer.nextToken(0.5, [2, 14], false),
  );

  // [output]
  yield* all(
    retreiveToken(6, 8, 0.5),
    graph.showJoin(5, 0.5),
    graph.highlightJoin(5, 0.5),
    tokenizer.nextToken(0.5, [-14, 6], false),
  );

  // [False]
  yield* all(
    retreiveToken(7, 10, 0.5),
    graph.showJoin(6, 0.5),
    graph.highlightJoin(6, 0.5),
    tokenizer.nextToken(0.5, [3, 5], false),
  );

  yield* all(
    graph.unhighlight(1),
    tokenizer.nextToken(1, [0, 0], false),
    example_code.cursorRect.opacity(0, 0.75),
  );

  yield* waitFor(1);

  yield* Chapters.spotOne([2], 1);
});
