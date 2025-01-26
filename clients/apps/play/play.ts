import {
  Color,
  Move,
  Nullable,
  position,
  engineCompute,
  engineMove,
  moveHist,
  inputNone,
} from "../lib/ucui/types";
import { playSound } from "./sound";
import { assign, dispatch, get } from "./store";
import { withQueryString } from "./util";

type Outcome = "½-½" | "1-0" | "0-1";
type MessageReady = {
  readonly _tag: "Ready";
  name: string;
  turn: Color;
  legalMoves: Move[];
};
type MessagePosition = { readonly _tag: "Position"; legalMoves: Move[] };
type MessageEngineMove = {
  readonly _tag: "EngineMove";
  move: Move;
  from: Move[];
  status: string;
};
type MessageOutcome = { readonly _tag: "Outcome"; outcome: Outcome };

let socket: Nullable<WebSocket> = null;

const socketURL = () => {
  const { position, engineColor, black, white } = get("gameConfig");
  const host = document.location.hostname;
  const proto = host !== "localhost" && host !== "127.0.0.1" ? "wss" : "ws";
  const port =
    document.location.port.length > 0 && document.location.port !== "8000"
      ? "8000"
      : document.location.port;
  const url =
    port.length > 0
      ? `${proto}://${host}:${port}/engine`
      : `${proto}://${host}/engine`;

  return withQueryString(url, {
    engine_color: engineColor,
    fen: position,
    white_time: white,
    black_time: black,
  });
};

const handleReady = (message: MessageReady) => {
  assign("started", true);
  assign("engineName", message.name);
  assign("position", position(message.legalMoves));
  if (message.turn === get("gameConfig").engineColor) {
    assign("engine", engineCompute());
  }
};
const handlePosition = (message: MessagePosition) => {
  console.log("handlePosition", message);
  assign("position", position(message.legalMoves));
};
const handleEngineMove = (message: MessageEngineMove) => {
  console.log("handleEngineMove", message);
  playSound();
  assign("engine", engineMove(message.move, message.from, message.status));
  dispatch("moveList", (list) =>
    list.concat(moveHist(message.move, message.from))
  );
};
const handleOutcome = (message: MessageOutcome) => {
  console.log("handleOutcome", message);
  console.log("Outcome", message.outcome);
  assign("started", false);
  assign("outcome", message.outcome);
  assign("screen", "movelist");
};

const handleIcoming = (event: MessageEvent) => {
  const message = JSON.parse(event.data);

  switch (message._tag) {
    case "Ready":
      return handleReady(message as MessageReady);
    case "Position":
      return handlePosition(message as MessagePosition);
    case "EngineMove":
      return handleEngineMove(message as MessageEngineMove);
    case "Outcome":
      return handleOutcome(message as MessageOutcome);
  }
};

const CONNECT_TIMEOUT = 4000;

// TODO
// const reConnect = () => {
//
// }

export const connect = () =>
  new Promise<string>((resolve, reject) => {
    const timeoutError = setTimeout(
      () => reject("Timeout error"),
      CONNECT_TIMEOUT
    );
    socket = new WebSocket(socketURL());
    socket.addEventListener("open", () => {
      clearTimeout(timeoutError);
      resolve("Ready");
    });
    socket.addEventListener("message", handleIcoming);
    socket.addEventListener("close", () => {
      socket = null;
      assign("started", false);
    });
  });

export const sendMove = (move: Move) => {
  if (get("started")) {
    if (socket === null) {
      console.error("sending move on null socket");
    } else {
      const clock = get("clock");
      assign("engine", engineCompute());
      if (clock._tag === "running") {
        socket.send(
          JSON.stringify({
            _tag: "Move",
            move,
            white_time: clock.remaining_white,
            black_time: clock.remaining_black,
          })
        );
        assign("input", inputNone());
      }
    }
  } else {
    console.error("game has not started");
  }
};
