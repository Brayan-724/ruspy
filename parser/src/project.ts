import { makeProject } from "@motion-canvas/core";

import intro from "./scenes/intro?scene";
import parser from "./scenes/parser?scene";
import tokens from "./scenes/tokens?scene";
import lexer from "./scenes/lexer?scene";

export default makeProject({
  scenes: [intro, parser, tokens, lexer],
});
