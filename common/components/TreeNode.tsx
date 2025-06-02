import { Layout, Rect, RectProps } from "@motion-canvas/2d";
import {
  PossibleVector2,
  SignalValue,
  unwrap,
  Vector2,
} from "@motion-canvas/core";
import { CustomBezier } from "./CustomBezier";
import { CodeColors } from "./CodeColors";

interface TreeNodeProps extends RectProps {
  child: Layout;
}

export class TreeNode extends Rect {
  constructor(props: TreeNodeProps) {
    super({
      radius: 12,
      // stroke: "#3E1C96",
      // stroke: "#FFF",
      stroke: CodeColors.WHITE,
      lineWidth: 3,
      fill: "#FF8637",
      width: () => props.child.width() + 50,
      height: () => props.child.height() + 25,
      ...props,
      children: props.child,
    });
  }

  public joinToPoint(point: SignalValue<PossibleVector2>): CustomBezier {
    const p0 = () => new Vector2(unwrap(point));
    const p3 = () => this.position().add([0, this.height() / -2]);

    const p_mid = () => p0().add(p3()).div(2);
    const p1 = () => p0().add([0, Math.abs(p_mid().y)]);
    const p2 = () => p3().sub([0, Math.abs(p_mid().y)]);

    return (
      <CustomBezier
        // stroke="#3E1C96"
        stroke="#FFF"
        lineWidth={10}
        p0={p0}
        p1={p1}
        p2={p2}
        p3={p3}
      />
    ) as CustomBezier;
  }
}
