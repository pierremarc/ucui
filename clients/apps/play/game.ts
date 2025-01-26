import { MoveHist, inputNone } from "../lib/ucui/types";
import { mountClock, startClock } from "./clock";
import { mountEngine } from "./engine";
import { mountInput } from "./input";
import { assign, defaultPosition, get } from "./store";

export const startGame = (moveList = [] as MoveHist[]) => {
  const { white, black } = get("gameConfig");
  assign("position", defaultPosition());
  assign("input", inputNone());
  assign("moveList", moveList);
  startClock(white, black);
};

export const mountGame = (root: HTMLElement) => {
  mountEngine(root);
  mountInput(root);
  mountClock(root);
};
