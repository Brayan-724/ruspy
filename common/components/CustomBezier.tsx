import {
  CubicBezier,
  CubicBezierProps,
  initial,
  PossibleCanvasStyle,
  Rect,
  signal,
} from "@motion-canvas/2d";
import { PossibleVector2, SignalValue, SimpleSignal } from "@motion-canvas/core";

export interface CustomBezierProps extends CubicBezierProps {
  debug?: SignalValue<boolean>;
  debugStroke?: SignalValue<PossibleCanvasStyle>;
}

export class CustomBezier extends CubicBezier {
  @initial(false)
  @signal()
  declare public readonly debug: SimpleSignal<boolean>;

  @initial("#F00")
  @signal()
  declare public readonly debugStroke: SimpleSignal<PossibleCanvasStyle>;

  constructor(props: CustomBezierProps) {
    super(props);

    if (this.debug()) {
      const common = {
        fill: this.debugStroke,
        width: 15,
        height: 15,
      };

      this.add(
        <>
          <Rect position={props.p0} {...common} />
          <Rect position={props.p1} {...common} />
          <Rect position={props.p2} {...common} />
          <Rect position={props.p3} {...common} />
        </>,
      );
    }
  }
}
