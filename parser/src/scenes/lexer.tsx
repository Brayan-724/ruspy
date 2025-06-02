import { makeScene2D } from "@motion-canvas/2d";
import { all, waitFor } from "@motion-canvas/core";
import { pyHighligther } from "ruspy-common/highlights";
import {
  Chapters,
  Code,
  CodeTokenizer,
  SceneLayout,
} from "ruspy-common/components";

import {
  EXAMPLE_CODE,
  EXAMPLE_CODE_2,
  EXAMPLE_CODE_TOKENS_ANIMATED,
} from "../commons";

export default makeScene2D(function* (view) {
  let example_code: Code;
  yield* SceneLayout.setup(view, function* () {
    yield* Chapters.spotOne([0], 0);
    yield* Chapters.focusOne([2, 3], [0, 2], 0);

    example_code = (
      <Code
        highlighter={pyHighligther}
        code={EXAMPLE_CODE}
      />
    ) as Code;

    example_code.cursorRect.opacity(0);

    view.add(example_code);
  });

  yield* all(
    example_code.code(EXAMPLE_CODE_2, 1),
    example_code.offset.x(-1, 1),
    example_code.letterSpacing(10, 1),
    waitFor(0.5, example_code.cursorRect.opacity(1, 0.5)),
  );

  example_code.position.x(() =>
    -example_code.letterSize().width * example_code.boxSelection.y()
  );

  const tokenizer = new CodeTokenizer(example_code);

  yield* tokenizer.tokenize(EXAMPLE_CODE_TOKENS_ANIMATED);

  yield* all(
    example_code.cursorRect.opacity(0, 0.15),
    example_code.x(0, 1),
    example_code.y(0, 1),
    example_code.offset.x(0, 1),
    example_code.letterSpacing(1, 1),
  );

  yield* waitFor(1);

  yield* Chapters.spotOne([0, 0], 1);
  yield* Chapters.spotOne([1], 1);
});
