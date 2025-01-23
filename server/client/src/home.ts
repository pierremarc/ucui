import { startGame } from "./game";
import { events } from "./lib/dom";
import { ANCHOR, DIV } from "./lib/html";
import { connect } from "./play";
import { assign } from "./store";

const buttonPlay = events(DIV("button-play", "play"), (add) =>
  add("click", () =>
    connect()
      .then(() => {
        startGame();
        assign("screen", "game");
      })
      .catch((err) => console.error("Connectin failed", err))
  )
);
const button_config = events(DIV("button-config", "config"), (add) =>
  add("click", () => assign("screen", "config"))
);

const footer = DIV(
  "footer",
  ANCHOR("link", "https://github.com/pierremarc/ucui", "Source code & feedback")
);

export const mountHome = (root: HTMLElement) => {
  root.append(DIV("home", buttonPlay, button_config, footer));
};
