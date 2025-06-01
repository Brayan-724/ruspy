import {
  AnimatedToken,
  CodeColors,
  InmediateToken,
} from "ruspy-common/components";

export const EXAMPLE_CODE = `\
if "Hi":
  output = True
else:
  output = False\
`;

export const EXAMPLE_CODE_2 = `\
if "Hi":\
 output = True \
else:\
 output = False\
`;

export const EXAMPLE_CODE_TOKENS: InmediateToken[] = [
  [[0, 2], CodeColors.BLUE],
  [[1, 4], CodeColors.GREEN],
  [[0, 1], CodeColors.WHITE],
  [[1, 6], CodeColors.CYAN],
  [[1, 1], CodeColors.GREEN],
  [[1, 4], CodeColors.ORANGE],
  [[1, 4], CodeColors.BLUE],
  [[0, 1], CodeColors.WHITE],
  [[1, 6], CodeColors.CYAN],
  [[1, 1], CodeColors.GREEN],
  [[1, 5], CodeColors.ORANGE],
];

export const EXAMPLE_CODE_TOKENS_ANIMATED: AnimatedToken[] = [
  [[0, 2], 1, CodeColors.BLUE],
  [[1, 4], 0.5, CodeColors.GREEN],
  [[0, 1], 0.4, CodeColors.WHITE],
  [[1, 6], 0.35, CodeColors.CYAN],
  [[1, 1], 0.30, CodeColors.GREEN],
  [[1, 4], 0.30, CodeColors.ORANGE],
  [[1, 4], 0.25, CodeColors.BLUE],
  [[0, 1], 0.25, CodeColors.WHITE],
  [[1, 6], 0.25, CodeColors.CYAN],
  [[1, 1], 0.20, CodeColors.GREEN],
  [[1, 5], 0.20, CodeColors.ORANGE],
];
