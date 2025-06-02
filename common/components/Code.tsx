import {
  Code as MotionCode,
  CodeProps as MotionCodeProps,
  initial,
  Rect,
  resolveScope,
  Txt,
  vector2Signal,
  word,
} from "@motion-canvas/2d";
import {
  all,
  createComputed,
  PossibleVector2,
  Promisable,
  SignalValue,
  Thread,
  ThreadGenerator,
  Vector2,
  Vector2Signal,
  waitFor,
} from "@motion-canvas/core";

import { CodeColors } from "./CodeColors";

export interface SnapshotBBox {
  start: number;
  end: number;
  x(): number;
  y(): number;
  width(): number;
  height(): number;
}

export interface CodeProps extends MotionCodeProps {
  boxSelection?: SignalValue<PossibleVector2>;
}

export class Code extends MotionCode {
  // x <- start letter
  // y <- end letter
  @initial([0, 0])
  @vector2Signal("boxSelection")
  public boxSelection: Vector2Signal;

  public readonly letterSize: () => Vector2;
  public readonly boxSelectionLength: () => number;

  public readonly cursorRect: Rect;

  constructor(props: CodeProps) {
    super(props);

    this.letterSize = createComputed(() =>
      this.getSelectionBBox(word(0, 0, 1))[0].size
    );
    this.boxSelectionLength = createComputed(() =>
      this.boxSelection.y() - this.boxSelection.x()
    );

    this.cursorRect = (
      <Rect
        x={() =>
          this.localLeft() + this.letterSize().width * this.boxSelection.x() -
          5}
        width={() => this.letterSize().width * this.boxSelectionLength() + 3}
        height={() => this.letterSize().height}
        offsetX={-1}
        stroke="#FFF"
        lineWidth={3}
        radius={12}
        zIndex={1}
      />
    ) as Rect;

    this.add(this.cursorRect);
  }

  localLeft(): number {
    return -this.width() / 2;
  }

  boxSelectionBBox() {
    const [start, end] = this.boxSelection().toArray();

    return {
      start,
      end,
      x: () => this.localLeft() + this.letterSize().width * start,
      y: () => 0,
      width: () => this.letterSize().width * (end - start),
      height: () => this.letterSize().height,
    } as SnapshotBBox;
  }

  highlightToken(background: CodeColors): Token;

  highlightToken(
    background: CodeColors,
    duration: number,
  ): TokenGenerator<Token>;

  highlightToken(
    background: CodeColors,
    duration?: number,
  ): Token | TokenGenerator<Token> {
    const bbox = this.boxSelectionBBox();
    const content = createComputed(() =>
      resolveScope(this.code(), true).substring(bbox.start, bbox.end)
    );

    const highlight = (
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
          y={-4}
          fill={CodeColors.text(background)}
          text={content}
          letterSpacing={() => this.letterSpacing()}
          fontSize={() => this.fontSize()}
          fontFamily={() => this.fontFamily()}
        />
      </Rect>
    ) as Rect;

    this.add(highlight);

    const token: Token = {
      span: [bbox.start, bbox.end],
      content,
      container: highlight,
    };

    function* t(): TokenGenerator<Token> {
      yield* highlight.end(1, duration);
      return token;
    }

    if (duration) {
      return t();
    } else {
      highlight.end(1);
      return token;
    }
  }
}

export interface Token {
  span: [number, number];
  content(): string;
  container: Rect;
}

export type InmediateToken = [
  pos: [number, number],
  color: CodeColors,
];

export type AnimatedToken = [
  pos: [number, number],
  duration: number,
  color: CodeColors,
];

export type TokenGenerator<TReturn = Token[]> = Generator<
  ThreadGenerator | Promise<any> | Promisable<any> | void,
  TReturn,
  Thread
>;

export class CodeTokenizer {
  public lastToken: number = 0;

  constructor(public readonly parent: Code) {}

  /** pos is relative to previous token */
  tokenize(tokens: InmediateToken[]): Token[];
  /** pos is relative to previous token */
  tokenize(
    tokens: AnimatedToken[],
  ): Generator<ThreadGenerator | void, Token[], Thread>;

  tokenize(
    tokens: InmediateToken[] | AnimatedToken[],
  ): Token[] | TokenGenerator {
    if (tokens.length && tokens[0].length === 3) {
      return this.tokenize_gen(tokens as AnimatedToken[]);
    } else {
      return this.tokenize_inm(tokens as InmediateToken[]);
    }
  }

  private tokenize_inm(tokens: InmediateToken[]): Token[] {
    let prevColor: CodeColors;
    const collect: Token[] = [];

    for (const [pos, color] of tokens) {
      if (prevColor) {
        collect.push(this.parent.highlightToken(prevColor));
      }
      this.nextToken(pos);
      prevColor = color;
    }

    if (prevColor) {
      collect.push(this.parent.highlightToken(prevColor));
      this.nextToken([0, 0]);
    }

    return collect;
  }

  private *tokenize_gen(tokens: AnimatedToken[]): TokenGenerator {
    let prevColor: CodeColors;
    let prevDuration: number;
    const collect: Token[] = [];

    const parent = this.parent;
    function* t(duration: number) {
      const rect = yield* parent.highlightToken(prevColor, duration / 2);
      collect.push(rect);
    }

    for (const [pos, duration, color] of tokens) {
      yield* all(
        prevColor ? t(duration) : undefined,
        this.nextToken(duration, pos),
      );

      prevColor = color;
      prevDuration = duration;
    }

    if (prevColor) {
      yield* all(
        t(prevDuration),
        this.nextToken(prevDuration, [0, 0]),
      );
    }

    return collect;
  }

  nextToken(next: [number, number]): void;
  nextToken(
    duration: number,
    next: [number, number],
    normalize?: boolean,
  ): ThreadGenerator;

  nextToken(
    durationOrNext: number | [number, number],
    maybeNext?: [number, number],
    normalize: boolean = true,
  ) {
    const hasDuration = typeof durationOrNext === "number";
    const next = hasDuration ? maybeNext! : durationOrNext;

    const newStart = this.lastToken + next[0];
    this.lastToken += next[0] + next[1];

    if (hasDuration) {
      const duration = durationOrNext!;

      if (normalize) {
        return all(
          this.parent.boxSelection.x(newStart, duration / 2),
          this.parent.boxSelection.y(newStart, duration / 2),
          waitFor(
            duration / 2,
            this.parent.boxSelection.y(this.lastToken, duration),
          ),
        );
      } else {
        return all(
          this.parent.boxSelection.x(newStart, duration),
          this.parent.boxSelection.y(this.lastToken, duration),
        );
      }
    } else {
      this.parent.boxSelection([newStart, this.lastToken]);
    }
  }
}
