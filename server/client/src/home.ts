import { events } from "./lib/dom";
import { DIV } from "./lib/html";
import { startGame } from "./play";
import { assign } from "./store";

export const mountHome = (root: HTMLElement) => {
  root.append(
    events(DIV("button-play", "play"), (add) =>
      add("click", () => {
        startGame();
        assign("screen", "game");
      })
    )
  );
};
