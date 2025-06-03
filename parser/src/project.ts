import { makeProject } from "@motion-canvas/core";
import { Chapters } from "ruspy-common/components";

import intro from "./scenes/intro?scene";
import introParser from "./scenes/intro-parser?scene";
import parser from "./scenes/parser?scene";
import lexer from "./scenes/lexer?scene";
import interpreter from "./scenes/interpreter?scene";

Chapters.configure([
  [
    "Lexer",
    "Structs",
    "Implement"
  ],
  "Parser",
  "Interpreter",
]);

export default makeProject({
  scenes: [
    intro,
    introParser,
    lexer,
    parser,
    interpreter
  ],
});
