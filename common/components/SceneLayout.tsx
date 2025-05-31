/// <reference types="vite/client" />

import {
  Img,
  Layout,
  LayoutProps,
  Rect,
  saturate,
  View2D,
} from "@motion-canvas/2d";
import { fadeTransition, ThreadGenerator } from "@motion-canvas/core";

import { Chapters } from "./Chapters";

import CrabsBack from "../../assets/crabs-back.png";
import RustLangEsLogo from "../../assets/Rust_Lang_ES_Logo.svg";

export class SceneLayout extends Layout {
  static *setup(view: View2D, preShow: (view: View2D) => ThreadGenerator = function*() {}) {
    view.add(<SceneLayout />);

    Chapters.useView(view);

    yield* preShow?.(view);

    yield* fadeTransition(1);
  }

  constructor(props: LayoutProps) {
    super(props);

    this.add(<Rect width={1920} height={1080} fill="#2E2E2E" />);

    this.add(
      <Img
        src={CrabsBack}
        width={1920}
        height={1080}
        opacity={0.05}
        filters={[saturate(0.3)]}
      />,
    );
    this.add(<Img src={RustLangEsLogo} x={-850} y={450} width={150} />);
  }
}
