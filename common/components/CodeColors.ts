export enum CodeColors {
  BLUE = "#26264d",
  GREEN = "#264d26",
  WHITE = "#4d4d4d",
  CYAN = "#264d4d",
  ORANGE = "#4d3e26",
}

export enum TextCodeColors {
  BLUE = "#5A7BA6",
  GREEN = "#9BB485",
  WHITE = "#FFF",
  CYAN = "#8FBCBB",
  ORANGE = "#D08770",
}

const TXT_COLORS: Record<CodeColors, TextCodeColors> = {
  [CodeColors.BLUE]: TextCodeColors.BLUE,
  [CodeColors.GREEN]: TextCodeColors.GREEN,
  [CodeColors.WHITE]: TextCodeColors.WHITE,
  [CodeColors.CYAN]: TextCodeColors.CYAN,
  [CodeColors.ORANGE]: TextCodeColors.ORANGE,
};

export namespace CodeColors {
  export function text(self: CodeColors): string {
    return TXT_COLORS[self];
  }
}
