import { startGame } from "./game";
import { events } from "./lib/dom";
import { ANCHOR, DIV, SPAN } from "./lib/html";
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

const intro = DIV(
  "intro",
  SPAN("ucui", "µcui "),
  `
  is there to train or play with a chess engine over the board, 
  when the purse can't afford a fine elctronic chess set. 
  It aims to be as little disruptive as possible, please 
  try and give us feedback. 
    `
);

export const mountHome = (root: HTMLElement) => {
  root.append(DIV("home", intro, buttonPlay, button_config, footer));
};
