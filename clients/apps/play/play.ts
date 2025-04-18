import {
  Color,
  Move,
  Nullable,
  position,
  engineCompute,
  engineMove,
  moveHist,
  inputNone,
  FEN_INITIAL_POSITION,
  EngineScore,
} from "../lib/ucui/types";
import { isPrivateIP } from "../lib/util";
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
type MessagePosition = {
  readonly _tag: "Position";
  legalMoves: Move[];
  fen: string;
};
type MessageEngineMove = {
  readonly _tag: "EngineMove";
  move: Move;
  from: Move[];
  check: string;
  fen: string;
  score: EngineScore;
};
type MessageOutcome = { readonly _tag: "Outcome"; outcome: Outcome };

let socket: Nullable<WebSocket> = null;

const socketURL = () => {
  const { fen, engineColor, black, white } = get("gameConfig");
  const host = document.location.hostname;
  const proto = isPrivateIP(host) ? "ws" : "wss";
  const port = document.location.port;
  const url =
    port.length > 0
      ? `${proto}://${host}:8000/engine` // if we run on vite dev server, we want to connect to ucui-server defqult port
      : `${proto}://${host}/engine`;

  return withQueryString(url, {
    engine_color: engineColor,
    fen,
    white_time: white,
    black_time: black,
  });
};

const handleReady = (message: MessageReady) => {
  const config = get("gameConfig");
  assign("started", true);
  assign("engineName", message.name);
  assign(
    "position",
    position(message.legalMoves, config.fen ?? FEN_INITIAL_POSITION)
  );
  if (message.turn === config.engineColor) {
    assign("engine", engineCompute());
  }
};
const handlePosition = (message: MessagePosition) => {
  console.debug("handlePosition", message);
  assign("position", position(message.legalMoves, message.fen));
};
const handleEngineMove = (message: MessageEngineMove) => {
  console.debug("handleEngineMove", message);
  playSound();
  assign(
    "engine",
    engineMove(message.move, message.from, message.score, message.check)
  );
  dispatch("moveList", (list) =>
    list.concat(moveHist(message.move, message.from, message.fen))
  );
};
const handleOutcome = (message: MessageOutcome) => {
  console.debug("handleOutcome", message);
  console.debug("Outcome", message.outcome);
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

export const disconnect = () => {
  socket?.close(1000, "end of game");
  socket = null;
};

export const connect = () =>
  new Promise<string>((resolve, reject) => {
    const timeoutError = setTimeout(
      () => reject("Timeout error"),
      CONNECT_TIMEOUT
    );
    socket = new WebSocket(socketURL());
    socket.addEventListener("message", handleIcoming);
    socket.addEventListener("close", (ev) => {
      console.log(`Socket closed for reason: ${ev.reason}`);
      socket = null;
      assign("started", false);
    });
    socket.addEventListener("open", () => {
      clearTimeout(timeoutError);
      resolve("Ready");
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
