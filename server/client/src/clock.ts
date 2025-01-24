import {
  addClass,
  DIV,
  hasClass,
  removeClass,
  replaceNodeContent,
} from "./lib/html";
import { iife } from "./lib/util";
import {
  assign,
  clockFlag,
  clockRunning,
  ClockState,
  dispatch,
  get,
  getTurn,
  Nullable,
  subscribe,
} from "./store";

/// exports

const white = DIV("time white active", "--:--");
const black = DIV("time black", "--:--");

export const mountClock = (root: Element) => {
  root.append(DIV("clock", white, black));
  renderClockTime();
  renderClockTurn();
  subscribe("clock")(renderClockTime);
  subscribe("moveList")(renderClockTurn);
};

// export const hitClock = () =>
//   dispatch("clock", (state) => {
//     if (state._tag == "running") {
//       toggleActive(white);
//       toggleActive(black);
//       return updateClock({ ...state, turn: otherColor(state.turn) });
//     }
//     return state;
//   });

let clockIt: Nullable<number> = null;

export const stopClock = () => clearInterval(clockIt ?? undefined);

export const startClock = (max_white: number, max_black: number) => {
  const start = Date.now();

  white_time = 0;
  white_max_time = max_white;
  black_time = 0;
  black_max_time = max_black;

  stopClock();

  clockIt = window.setInterval(
    () =>
      dispatch("clock", (state) => {
        const newState = updateClock(state);
        if (newState._tag === "flag") {
          stopClock();
        }
        return newState;
      }),
    100
  );
  assign("clock", clockRunning(start, 0, 0));
};

/// impl

// Our bit of internal state
let white_time = 0;
let black_time = 0;
let white_max_time = 0;
let black_max_time = 0;

const { floor } = Math;

const formatTime = (millis: number) => {
  const seconds = millis / 1000;
  const sec = floor(seconds % 60);
  const minutes = floor((seconds / 60) % 60);
  const hours = floor(seconds / 60 / 60);

  const fs = sec < 10 ? `0${sec.toFixed(0)}` : `${sec.toFixed(0)}`;
  const fm = minutes < 10 ? `0${minutes.toFixed(0)}` : `${minutes.toFixed(0)}`;
  const fh = hours < 10 ? `0${hours.toFixed(0)}` : `${hours.toFixed(0)}`;

  return seconds >= 3600 ? `${fh}:${fm}:${fs}` : `${fm}:${fs}`;
};

const r = removeClass("active");
const a = addClass("active");
const h = hasClass("active");
const toggleActive = (e: HTMLElement, turn: boolean) =>
  turn && !h(e) ? a(e) : r(e);

const renderClockTurn = () => {
  const turn = getTurn();
  toggleActive(white, turn == "white");
  toggleActive(black, turn == "black");
};

const renderClockTime = () => {
  const setWhite = replaceNodeContent(white);
  const setBlack = replaceNodeContent(black);

  const state = get("clock");
  switch (state._tag) {
    case "flag": {
      if (state.color === "white") {
        setWhite("flag");
        setBlack(formatTime(state.other));
      } else {
        setBlack("flag");
        setWhite(formatTime(state.other));
      }
      break;
    }
    case "running": {
      setBlack(formatTime(state.remaining_black));
      setWhite(formatTime(state.remaining_white));
      break;
    }
    case "initial": {
      setBlack("--:--");
      setWhite("--:--");
      break;
    }
  }
};

const updateClock = (state: Readonly<ClockState>) => {
  if (state._tag === "running") {
    let now = Date.now();
    let total_spent = white_time + black_time;
    let total = now - state.start_time;
    let inc = total - total_spent;

    iife(() => {
      switch (getTurn()) {
        case "white":
          return (white_time += inc);
        case "black":
          return (black_time += inc);
      }
    });

    if (black_time >= black_max_time) {
      return clockFlag("black", white_max_time - white_time);
    } else if (white_time >= white_max_time) {
      return clockFlag("white", black_max_time - black_time);
    } else {
      return clockRunning(
        state.start_time,
        white_max_time - white_time,
        black_max_time - black_time
      );
    }
  }
  return { ...state };
};
