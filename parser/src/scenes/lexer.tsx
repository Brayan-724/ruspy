import { Code, makeScene2D, Rect, Txt, word } from "@motion-canvas/2d";
import { all, ThreadGenerator, Vector2, waitFor } from "@motion-canvas/core";
import { pyHighligther } from "ruspy-common/highlights";
import { Chapters, SceneLayout } from "ruspy-common/components";

import { EXAMPLE_CODE, EXAMPLE_CODE_2 } from "../commons";

enum CodeColors {
  BLUE = "#26264d",
  GREEN = "#264d26",
  WHITE = "#4d4d4d",
  CYAN = "#264d4d",
  ORANGE = "#4d3e26",
}

const TXT_BLUE = "#5A7BA6";
const TXT_GREEN = "#9BB485";
const TXT_WHITE = "#FFF";
const TXT_CYAN = "#8FBCBB";
const TXT_ORANGE = "#D08770";

const TXT_COLORS: Record<CodeColors, string> = {
  [CodeColors.BLUE]: TXT_BLUE,
  [CodeColors.GREEN]: TXT_GREEN,
  [CodeColors.WHITE]: TXT_WHITE,
  [CodeColors.CYAN]: TXT_CYAN,
  [CodeColors.ORANGE]: TXT_ORANGE,
};

interface SnapshotBBox {
  start: number;
  end: number;
  x(): number;
  y(): number;
  width(): number;
  height(): number;
}

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

    view.add(example_code);
  });

  yield* all(
    example_code.code(EXAMPLE_CODE_2, 1),
    example_code.offset.x(-1, 1),
    example_code.letterSpacing(10, 1),
  );

  const letter_size = () =>
    example_code.getSelectionBBox(word(0, 0, 1))[0].width;

  // x <- start letter
  // y <- end letter
  const selection = Vector2.createSignal([0, 0]);
  const length = () => selection.y() - selection.x();

  example_code.position.x(() => -letter_size() * selection.y());

  const snapshotBBox = () => {
    const [start, end] = selection().toArray();

    return {
      start,
      end,
      x: () => example_code.left().x + letter_size() * start,
      y: () => example_code.position.y(),
      width: () => letter_size() * (end - start),
      height: () => example_code.height(),
    } as SnapshotBBox;
  };

  function highlightToken(
    background: CodeColors,
    duration: number = 1,
  ): ThreadGenerator {
    const bbox = snapshotBBox();

    const underline = (
      <Rect
        x={bbox.x}
        y={bbox.y}
        width={bbox.width}
        offsetX={-1}
        fill={background}
        stroke={CodeColors.WHITE}
        end={0}
        lineWidth={3}
        radius={12}
        height={bbox.height}
      >
        <Txt
          y={-3}
          fill={TXT_COLORS[background]}
          text={EXAMPLE_CODE_2.substring(bbox.start, bbox.end)}
          letterSpacing={() => example_code.letterSpacing()}
          fontSize={() => example_code.fontSize()}
          fontFamily={() => example_code.fontFamily()}
        />
      </Rect>
    ) as Rect;

    view.add(underline);
    underline.moveBelow(example_code);

    return underline.end(1, duration);
  }

  let lastToken = 0;
  const nextToken = function* (
    color: CodeColors,
    duration: number,
    next: [number, number],
  ): ThreadGenerator {
    yield* all(
      highlightToken(color, duration / 2),
      selection.x(lastToken + next[0], duration / 2),
      selection.y(lastToken + next[0], duration / 2),
    );

    lastToken += next[0] + next[1];

    yield* selection.y(lastToken, duration);
  };

  const cursor = (
    <Rect
      x={() => -letter_size() * length() - 5}
      width={() => letter_size() * length() + 3}
      height={example_code.height}
      offsetX={-1}
      stroke="#FFF"
      lineWidth={3}
      radius={12}
    />
  );

  view.add(cursor);

  yield* all(
    selection.y(2, 1),
  );

  lastToken = 2;
  yield* nextToken(CodeColors.BLUE, 0.50, [1, 4]);
  yield* nextToken(CodeColors.GREEN, 0.40, [0, 1]);
  yield* nextToken(CodeColors.WHITE, 0.35, [1, 6]);
  yield* nextToken(CodeColors.CYAN, 0.30, [1, 1]);
  yield* nextToken(CodeColors.GREEN, 0.30, [1, 4]);
  yield* nextToken(CodeColors.ORANGE, 0.25, [1, 4]);
  yield* nextToken(CodeColors.BLUE, 0.25, [0, 1]);
  yield* nextToken(CodeColors.WHITE, 0.25, [1, 6]);
  yield* nextToken(CodeColors.CYAN, 0.20, [1, 1]);
  yield* nextToken(CodeColors.GREEN, 0.20, [1, 5]);
  yield* all(
    nextToken(CodeColors.ORANGE, 0.20, [0, 0]),
    cursor.opacity(0, 0.15),
  );

  yield* all(
    example_code.x(0, 1),
    example_code.y(0, 1),
    example_code.offset.x(0, 1),
    example_code.letterSpacing(1, 1),
  );

  yield* waitFor(1);
});
