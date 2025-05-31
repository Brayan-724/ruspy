import { Layout, Rect, Txt, View2D } from "@motion-canvas/2d";
import { all, ThreadGenerator } from "@motion-canvas/core";

export type ChapterSpec = string | ChapterSpec[];

interface ChapterParent {
  width(): number;
  height(): number;
}

export abstract class Chapters {
  static totalChapters: number;
  static chapters: Chapter[];
  static chaptersNode: Chapter;
  static chaptersSpec: ChapterSpec[];

  static layout: Layout = null;

  private static countChapters(spec: ChapterSpec) {
    if (spec instanceof Array) {
      spec.forEach(Chapters.countChapters);
    } else {
      Chapters.totalChapters += 1;
    }
  }

  static configure(spec: ChapterSpec[]) {
    Chapters.countChapters(spec);
    Chapters.chaptersSpec = spec;
  }

  static useView(view: View2D) {
    this.layout = new Layout({
      x: -750,
      height: 20,
      offset: [-1, 1],
      width: () => view.width() - 200,
      y: () => view.height() / 2,
    });

    this.chaptersNode = new Chapter({
      spec: this.chaptersSpec,
      xOffset: () => -this.layout.width() / 2,
      parent: {
        width: () => this.layout.width(),
        height: () => this.layout.height(),
      },
    });

    this.chapters = this.chaptersNode.chapters;

    this.layout.add(this.chaptersNode);

    view.add(this.layout);
  }

  static getPath(path: number[]): Chapter {
    let base = Chapters.chaptersNode;

    for (const segment of path) {
      base = base.chapters[segment];
    }

    return base;
  }

  static *focusOne(ratio: [number, number], path: number[], duration: number) {
    const childIdx = path.pop();
    const parent = this.getPath(path);

    yield* parent.focus(ratio, [childIdx], duration);
  }

  static *resetAlign(path: number[], duration: number) {
    yield* this.getPath(path).resetAlign(duration);
  }

  static *spotOne(path: number[], duration: number) {
    const childIdx = path.pop();
    const parent = this.getPath(path);

    yield* parent.spot([childIdx], duration);
  }

  static *slideIn(duration: number) {
    yield* this.chaptersNode.slideIn(duration);
  }

  static *slideOut(duration: number) {
    yield* this.chaptersNode.slideOut(duration);
  }

  static *realignTxt(duration: number) {
    yield* this.chaptersNode.realignTxt(duration);
  }
}

export class Chapter extends Layout {
  readonly isGroup: boolean;
  readonly parentSpec: ChapterParent;

  readonly txt?: Txt;
  readonly rect?: Rect;
  readonly chapters: Chapter[] = [];
  readonly totalChapters: number = 1;

  constructor(
    { spec, parent, xOffset }: {
      spec: ChapterSpec;
      parent: ChapterParent;
      xOffset(): number;
    },
  ) {
    super({
      width: () => parent.width(),
      height: () => parent.height(),
      x: () => xOffset(),
      y: () => parent.height() / 2,
      offset: [-1, 1],
    });

    this.isGroup = spec instanceof Array;
    this.parentSpec = parent;

    if (this.isGroup) {
      const children: Chapter[] = [];
      let totalChapters = 0;

      let child: Chapter;
      for (const node of spec) {
        child = new Chapter({
          spec: node,
          xOffset: child?.rightSide?.bind?.(child) ??
            (() => -this.width() / 2),
          parent: {
            width: () => this.width(),
            height: () => this.height(),
          },
        });

        totalChapters += child.totalChapters;

        child.width(() => this.width() / spec.length - this.chapterGap());

        this.add(child);
        children.push(child);
      }

      this.chapters = children;
      this.totalChapters = totalChapters;
    } else {
      this.rect = (
        <Rect
          offset={[-1, 1]}
          x={() => -this.width() / 2}
          y={() => this.height() / 2}
          width={() => this.width()}
          height={this.parentSpec.height}
          fill="#FF8637"
          radius={0}
        />
      ) as Rect;

      this.txt = (
        <Txt
          opacity={() => Math.max(0, Math.min(1, this.width() / 30))}
          fontSize={42}
          offset={[0, 1]}
          y={-25}
          fill="white"
          fontFamily="Inter"
          text={spec as string}
        />
      ) as Txt;

      this.add([this.txt, this.rect]);
    }
  }

  chapterGap(): number {
    return this.isGroup ? 15 : Math.min(0, Math.max(15, this.width())) + 15;
  }

  rightSide(): number {
    return this.x() + this.width() + (this.isGroup ? 0 : 15);
  }

  *slideIn(duration: number): ThreadGenerator {
    if (this.isGroup) {
      yield* all(...this.chapters.map((c) => c.slideIn(duration)));
    } else {
      yield* all(
        this.txt.opacity(
          () => Math.max(0, Math.min(1, this.width() / 30)),
          duration,
        ),
        this.txt.scale(
          () => Math.max(0, Math.min(1, this.width() / 150)),
          duration,
        ),
        this.rect.offset.y(1, duration),
      );
    }
  }

  *slideOut(duration: number): ThreadGenerator {
    if (this.isGroup) {
      yield* all(...this.chapters.map((c) => c.slideOut(duration)));
    } else {
      yield* all(
        this.txt.opacity(0, duration),
        this.rect.offset.y(-1, duration),
      );
    }
  }

  *realignTxt(duration: number): ThreadGenerator {
    if (this.isGroup) {
      yield* all(...this.chapters.map((c) => c.realignTxt(duration)));
    } else {
      yield* all(
        this.txt.fontSize(42, duration),
        this.txt.offset([0, 1], duration),
        this.txt.position([0, -25], duration),
      );
    }
  }

  *focus(ratio: [number, number], path: number[], duration: number) {
    const target = path.shift();

    let actions: ThreadGenerator[] = [];

    for (const [idx, chapter] of this.chapters.entries()) {
      if (idx == target) {
        actions.push(
          chapter.width(
            () =>
              Chapters.chaptersNode.width() / (ratio[1] / ratio[0]) -
              Chapters.chaptersNode.chapterGap(),
            duration,
          ),
        );
      } else {
        actions.push(
          chapter.width(
            () =>
              Chapters.chaptersNode.width() / (ratio[0] * ratio[1]) -
              Chapters.chaptersNode.chapterGap(),
            duration,
          ),
        );
      }
    }

    yield* all(...actions);
  }

  *resetAlign(duration: number): ThreadGenerator {
    const actions: ThreadGenerator[] = [];

    for (const chapter of this.chapters) {
      actions.push(
        chapter.width(
          () => this.width() / this.chapters.length - this.chapterGap(),
          duration,
        ),
      );
      actions.push(chapter.resetAlign(duration));
    }

    yield* all(...actions);
  }

  *spot(path: number[], duration: number): ThreadGenerator {
    const target = path.shift();

    const actions: ThreadGenerator[] = [];

    for (const [idx, chapter] of this.chapters.entries()) {
      if (idx == target) {
        actions.push(
          chapter.width(() => this.width() - this.chapterGap(), duration),
        );
      } else {
        actions.push(chapter.width(0, duration));
      }
    }

    yield* all(...actions);
  }
}
