import {
  Layout,
  LayoutProps,
  Node,
  Rect,
  RectProps,
  signal,
  Txt,
} from "@motion-canvas/2d";
import {
  all,
  createEffect,
  createSignal,
  sequence,
  SignalValue,
  SimpleSignal,
} from "@motion-canvas/core";

import { CustomBezier } from "./CustomBezier";

export type TreeGraphJoin = [number, number] | [number, number, string] | [
  number,
  number,
  Node,
];

export interface TreeGraphProps extends LayoutProps {
  nodes: SignalValue<Node[]>;
  joins: SignalValue<TreeGraphJoin[]>;
}

function* dimElement(node: Node, duration: number) {
  if (node.opacity() !== 0) {
    yield* node.opacity(0.25, duration);
  }
}

type InternalJoin = [from: Node, to: Node, curve: CustomBezier, text?: Node];
export class TreeGraph extends Layout {
  @signal()
  declare public readonly nodes: SimpleSignal<Layout[]>;

  @signal()
  declare public readonly joins: SimpleSignal<TreeGraphJoin[]>;

  public readonly _joinNodes: SimpleSignal<InternalJoin[]> = createSignal([]);

  public joinNodes() {
    return this._joinNodes();
  }

  constructor(props: TreeGraphProps) {
    super(props);

    createEffect(() => {
      this.removeChildren();
      const joinNodes: InternalJoin[] = [];

      const nodes = this.nodes();
      this.add(nodes);

      for (const [from, to, content] of this.joins()) {
        const from_node = nodes[from];
        const to_node = nodes[to];

        const p0 = () => from_node.position().add([0, from_node.height() / 2]);
        const p3 = () => to_node.position().sub([0, to_node.height() / 2]);

        const p_mid = () => p0().add(p3()).div(2);
        const p1 = () => p0().add([0, Math.abs(p_mid().y)]);
        const p2 = () => p3().sub([0, Math.abs(p_mid().y)]);

        const node = (
          <CustomBezier
            stroke="#3E1C96"
            lineWidth={10}
            p0={p0}
            p1={p1}
            p2={p2}
            p3={p3}
          />
        ) as CustomBezier;

        let text: Node = <Node />;

        this.add(node);

        if (typeof content === "string") {
          text = (
            <Txt.b
              position={p_mid}
              text={content}
              fontFamily="Inter"
              fontSize={32}
              fill="#FFF6ED"
            />
          );
          this.add(text);
        } else if (content) {
          createEffect(() => {
            content.position(p_mid());
          });
          this.add(content);
          text = content;
        }

        joinNodes.push([from_node, to_node, node, text]);
      }

      this._joinNodes(joinNodes);
    });
  }

  public *showJoin(idx: number, duration: number, delay: number = 0.25) {
    const common = [
      all(
        this.joinNodes()[idx][2].end(1, duration),
        this.joinNodes()[idx][3].opacity(1, duration),
      ),
      this.joinNodes()[idx][1].opacity(1, duration),
    ];
    if (this.joinNodes()[idx][0].opacity() == 0) {
      yield* sequence(
        delay,
        this.joinNodes()[idx][0].opacity(1, duration),
        ...common,
      );
    } else {
      yield* sequence(delay, ...common);
    }
  }

  public *highlightNode(idx: number, duration: number) {
    const actions = this.joinNodes().flatMap((j) =>
      j.map((n) => dimElement(n, duration))
    );

    actions.push(this.nodes()[idx].opacity(1, duration));

    yield* all(...actions);
  }

  public *highlightJoin(idx: number, duration: number) {
    const actions = [];

    for (const [joinIdx, join] of this.joinNodes().entries()) {
      if (joinIdx === idx) {
        continue;
      }

      actions.push(
        dimElement(join[0], duration),
        dimElement(join[1], duration),
        dimElement(join[2], duration),
        dimElement(join[3], duration),
      );
    }

    actions.push(
      this.joinNodes()[idx][0].opacity(1, duration),
      this.joinNodes()[idx][1].opacity(1, duration),
      this.joinNodes()[idx][2].opacity(1, duration),
      this.joinNodes()[idx][3].opacity(1, duration),
    );

    yield* all(...actions);
  }

  public *unhighlight(duration: number) {
    yield* all(
      ...this.joinNodes().flatMap((j) => j.map((n) => n.opacity(1, duration))),
    );
  }
}
