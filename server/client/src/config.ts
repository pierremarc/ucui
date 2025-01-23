import { startGame } from "./game";
import { attrs, events } from "./lib/dom";
import { DIV, INPUT } from "./lib/html";
import { connect } from "./play";
import { assign, dispatch, get } from "./store";

const MIN_TIME = "00:01";
const MAX_TIME = "02:00";

const { floor } = Math;

const play = () =>
  connect()
    .then(() => {
      startGame();
      assign("screen", "game");
    })
    .catch((err) => console.error("Connectin failed", err));

const buttonPlay = events(DIV("button-play", "play"), (add) =>
  add("click", play)
);

const formatTime = (millis: number) => {
  const seconds = millis / 1000;
  const minutes = floor((seconds / 60) % 60);
  const hours = floor(seconds / 60 / 60);

  const fm = minutes < 10 ? `0${minutes.toFixed(0)}` : `${minutes.toFixed(0)}`;
  const fh = hours < 10 ? `0${hours.toFixed(0)}` : `${hours.toFixed(0)}`;

  return `${fh}:${fm}`;
};

const parseTime = (fmt: string) => {
  const [hours, minutes] = fmt.split(":").map((x) => parseInt(x));
  return (hours * 60 * 60 + minutes * 60) * 1000;
};

const inputTime = (t: number, onChange: (t: number) => void) =>
  events(
    attrs(INPUT("input-time", "time"), (set) => {
      set("min", MIN_TIME);
      set("max", MAX_TIME);
      set("value", formatTime(t));
    }),
    (add) =>
      add("change", (ev) => {
        const input = ev.currentTarget as HTMLInputElement;
        onChange(parseTime(input.value));
      })
  );

export const mountConfig = (root: HTMLElement) => {
  const config = get("gameConfig");
  const whiteInput = inputTime(config.white, (white) =>
    dispatch("gameConfig", (state) => ({ ...state, white }))
  );
  const blackInput = inputTime(config.black, (black) =>
    dispatch("gameConfig", (state) => ({ ...state, black }))
  );

  const fen = INPUT("input-fen", "text");

  const okFen = events(DIV("ok-button", "Start with position"), (add) =>
    add("click", () => {
      dispatch("gameConfig", (state) => ({ ...state, position: fen.value }));
      play();
    })
  );

  root.append(
    DIV(
      "config",
      DIV(
        "times",

        DIV("time", "white: ", whiteInput),
        DIV("time", "black: ", blackInput)
      ),
      DIV(
        "position",
        DIV("help", "Starting posititon in FEN format."),
        DIV("fen-box", fen, okFen)
      ),
      buttonPlay
    )
  );
};
