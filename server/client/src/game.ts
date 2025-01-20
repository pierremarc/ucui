import { mountClock, startClock } from "./clock";
import { startingLegalMoves } from "./data";
import { mountEngine } from "./engine";
import { mountInput } from "./input";
import { assign, inputNone, position } from "./store";

const startGame = () => {
  assign("position", position("white", startingLegalMoves));
  assign("input", inputNone());
  assign("moveList", []);
};

export const mountGame = (root: HTMLElement) => {
  mountEngine(root);
  mountInput(root);
  mountClock(root);

  startClock(60 * 10 * 1000, 60 * 1000);
  startGame();

  //   root.append(
  //     attrs(
  //       events(DIV("ml", "MOVELIST"), (add) =>
  //         add("click", () => assign("screen", "movelist"))
  //       ),
  //       (set) =>
  //         set("style", "position:absolute; top:46px; z-index:1111; color:green;")
  //     )
  //   );
};
