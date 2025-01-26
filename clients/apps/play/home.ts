import { events } from "../lib/dom";
import { DIV, ANCHOR, SPAN } from "../lib/html";
import { startGame } from "./game";
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
const buttonConfig = events(DIV("button-config", "config"), (add) =>
  add("click", () => assign("screen", "config"))
);

const buttonHistory = events(DIV("button-history", "my games"), (add) =>
  add("click", () => assign("screen", "history"))
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
  when the purse can't afford a fine electronic chess set. 
  It aims to be as little disruptive as possible, please 
  try and give us feedback. 
    `
);

export const mountHome = (root: HTMLElement) => {
  root.append(
    DIV("home", intro, buttonPlay, buttonConfig, buttonHistory, footer)
  );
};
