import { startGame } from "./game";
import { events } from "./lib/dom";
import { DIV } from "./lib/html";
import { connect } from "./play";
import { assign } from "./store";

export const mountHome = (root: HTMLElement) => {
  root.append(
    events(DIV("button-play", "play"), (add) =>
      add("click", () =>
        connect()
          .then(() => {
            startGame(10 * 60 * 1000, 60 * 1000);
            assign("screen", "game");
          })
          .catch((err) => console.error("Connectin failed", err))
      )
    )
  );
};
