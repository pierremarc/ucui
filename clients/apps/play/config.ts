import { events, attrs } from "../lib/dom";
import { DIV, H2, INPUT, replaceNodeContent } from "../lib/html";
import { otherColor } from "../lib/ucui/types";
import { renderEco } from "./eco";
import { startNewGame } from "./game";
import { connect } from "./play";
import { assign, dispatch, get, subscribe } from "./store";

const MIN_TIME = "00:01";
const MAX_TIME = "02:00";

const { floor } = Math;

const play = () =>
  connect()
    .then(() => {
      startNewGame();
      assign("screen", "game");
    })
    .catch((err) => console.error("Connectin failed", err));

const buttonPlay = () =>
  events(DIV("button-play", "play"), (add) => add("click", play));

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

const renderFen = () => {
  const fenInput = INPUT("input-fen", "text");

  const fenOk = events(DIV("ok-button", "Start with position"), (add) =>
    add("click", () => {
      dispatch("gameConfig", (state) => ({
        ...state,
        fen: fenInput.value,
      }));
      play();
    })
  );

  return DIV(
    "position",
    DIV("help", "Starting posititon in FEN format."),
    DIV("fen-box", fenInput, fenOk)
  );
};

const header = () =>
  DIV(
    "header",
    H2("title", "Game settings"),
    events(DIV("to-home  to-button", "↩"), (add) =>
      add("click", () => assign("screen", "home"))
    )
  );

export const mountConfig = (root: HTMLElement) => {
  const config = get("gameConfig");

  const engineColorInput = events(
    DIV(`color ${config.engineColor}`, config.engineColor),
    (add) =>
      add("click", () => {
        const engineColor = otherColor(get("gameConfig").engineColor);
        dispatch("gameConfig", (state) => ({
          ...state,
          black: state.white,
          white: state.black,
          engineColor,
        }));
        engineColorInput.classList.remove(otherColor(engineColor));
        engineColorInput.classList.add(engineColor);
        replaceNodeContent(engineColorInput)(engineColor);
      })
  );

  const whiteTimeInput = inputTime(config.white, (white) =>
    dispatch("gameConfig", (state) => ({ ...state, white }))
  );
  const blackTimeInput = inputTime(config.black, (black) =>
    dispatch("gameConfig", (state) => ({ ...state, black }))
  );

  subscribe("gameConfig")(() => {
    const { white, black } = get("gameConfig");
    whiteTimeInput.value = formatTime(white);
    blackTimeInput.value = formatTime(black);
  });

  root.append(
    DIV(
      "config",
      header(),
      DIV(
        "main",
        DIV("engine-color", DIV("label", "Engine color"), engineColorInput),
        DIV(
          "times",

          DIV("time", "White time ", whiteTimeInput),
          DIV("time", "Black time ", blackTimeInput)
        ),

        buttonPlay()
      ),
      renderEco(),
      renderFen()
    )
  );
};
