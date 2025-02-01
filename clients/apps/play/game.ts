import { addClass, DIV, removeClass } from "../lib/html";
import { MoveHist, inputNone } from "../lib/ucui/types";
import { mountClock, startClock } from "./clock";
import { mountEngine } from "./engine";
import { mountInput } from "./input";
import { assign, get, subscribe } from "./store";

export const startNewGame = () => {
  const { white, black } = get("gameConfig");
  assign("input", inputNone());
  assign("moveList", []);
  startClock(white, black);
};

export const startGameWithMoves = (moveList: MoveHist[]) => {
  const { white, black } = get("gameConfig");
  assign("moveList", moveList);
  assign("input", inputNone());
  startClock(white, black);
};

const mountLock = (root: HTMLElement) => {
  const lock = DIV("lock locked");
  const setLock = addClass("locked");
  const delLock = removeClass("locked");
  const update = () => (get("lockScreen") ? setLock(lock) : delLock(lock));
  const sub = subscribe("lockScreen");
  sub(update);
  update();
  root.append(lock);
};

export const mountGame = (root: HTMLElement) => {
  mountEngine(root);
  mountInput(root);
  mountClock(root);
  mountLock(root);
};
