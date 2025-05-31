import { makeScene2D } from "@motion-canvas/2d";
import { all, waitFor } from "@motion-canvas/core";
import { Chapters, SceneLayout } from "ruspy-common/components";

import "../fonts.css";

export default makeScene2D(function* (view) {
  yield* SceneLayout.setup(view, function* () {
    // setting up chapters
    yield* Chapters.spotOne([0, 0], 0);
    yield* Chapters.slideOut(0);
  });

  const viewCenter = view.size().div(2);

  const makeText = (path: number[]) => {
    const node = Chapters.getPath(path).txt;

    node.opacity(1);
    node.scale(0);
    node.offset([0, 0]);
    node.fontSize(58);
    node.absolutePosition(viewCenter);

    return node;
  };

  // SHOW LEXER

  const lexerTxt = makeText([0, 0]);

  yield* lexerTxt.opacity(1, 1);
  yield* lexerTxt.scale(1, 1);

  const textSpacing = lexerTxt.cacheBBox().height + 10;

  // SHOW PARSER

  const parserTxt = makeText([1]);

  yield* all(
    lexerTxt.absolutePosition(viewCenter.addY(-textSpacing), 1),
    parserTxt.scale(1, 1),
  );

  // SHOW INTERPRETER

  const runTxt = makeText([2]);

  yield* all(
    lexerTxt.absolutePosition(viewCenter.addY(-textSpacing * 2), 1),
    parserTxt.absolutePosition(viewCenter.addY(-textSpacing), 1),
    runTxt.scale(1, 1),
  );

  // SHOW CHAPTERS

  yield* all(
    Chapters.realignTxt(1),
    Chapters.slideIn(1),
  );

  yield* all(
    Chapters.spotOne([0], 1),
    Chapters.focusOne([2, 3], [0, 1], 1),
  );

  yield* waitFor(1);

  yield* Chapters.focusOne([2, 3], [0, 2], 1);
});
