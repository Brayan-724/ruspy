import { Rect, RectProps, signal, Txt, TxtProps } from "@motion-canvas/2d";
import { SignalValue, SimpleSignal } from "@motion-canvas/core";
import { CodeColors } from "./CodeColors";

export interface TokenProps extends Omit<RectProps, "fill"> {
  color: SignalValue<CodeColors>;
  text: SignalValue<string>;
  textProps?: Omit<TxtProps, "text">;
}

export class TokenBox extends Rect {
  @signal()
  declare public text: SimpleSignal<string>;

  @signal()
  declare public color: SimpleSignal<CodeColors>;

  readonly textNode: Txt;

  constructor(props: TokenProps) {
    super({
      stroke: CodeColors.WHITE,
      lineWidth: 3,
      radius: 12,
      fill: props.color,
      ...props,
    });

    if (this.getChildren().length === 0) {
      this.textNode = (
        <Txt
          y={-4}
          fill={() => CodeColors.text(this.color())}
          text={() => this.text()}
          {...props.textProps}
        />
      ) as Txt;

      this.add(this.textNode);

      if (this.width.isInitial()) {
        this.width(() => this.textNode.width() + 3);
      }

      if (this.height.isInitial()) {
        this.height(() => this.textNode.height());
      }
    } else {
      this.textNode = this.getChildren().find((n) => n instanceof Txt) ??
        this.getChildren()[0] as Txt;

      this.width(() => this.textNode.width() + 3);
      this.height(() => this.textNode.height());
    }
  }
}
