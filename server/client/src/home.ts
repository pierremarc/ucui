import { startGame } from "./game";
import { events } from "./lib/dom";
import { DIV } from "./lib/html";
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

export const mountHome = (root: HTMLElement) => {
  const button_config = events(DIV("button-config", "config"), (add) =>
    add("click", () => assign("screen", "config"))
  );

  root.append(DIV("home", buttonPlay, button_config));
};
