import { mountClock, startClock } from "./clock";
import { startingLegalMoves } from "./data";
import { mountEngine } from "./engine";
import { mountInput } from "./input";
import { assign, get, inputNone, MoveHist, position } from "./store";

export const startGame = (moveList = [] as MoveHist[]) => {
  const { white, black } = get("gameConfig");
  assign("position", position("white", startingLegalMoves));
  assign("input", inputNone());
  assign("moveList", moveList);
  startClock(white, black);
};

export const mountGame = (root: HTMLElement) => {
  mountEngine(root);
  mountInput(root);
  mountClock(root);
};
