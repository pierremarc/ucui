import { mountClock, startClock } from "./clock";
import { startingLegalMoves } from "./data";
import { mountEngine } from "./engine";
import { mountInput } from "./input";
import { assign, inputNone, position } from "./store";
import { connect } from "./play";

export const startGame = (
  white_time_millis: number,
  black_time_millis: number
) => {
  assign("position", position("white", startingLegalMoves));
  assign("input", inputNone());
  assign("moveList", []);
  startClock(white_time_millis, black_time_millis);
};

export const mountGame = (root: HTMLElement) => {
  mountEngine(root);
  mountInput(root);
  mountClock(root);
};
