import { hitClock } from "./clock";
import {
  assign,
  dispatch,
  engineCompute,
  engineMove,
  get,
  inputNone,
  Move,
  moveHist,
  Nullable,
  position,
} from "./store";

type Outcome = "½-½" | "1-0" | "0-1";
type MessageReady = { readonly _tag: "Ready" };
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
  const host = document.location.hostname;
  const proto = host !== "localhost" && host !== "127.0.0.1" ? "wss" : "ws";
  const port =
    document.location.port.length > 0 && document.location.port !== "8000"
      ? "8000"
      : document.location.port;
  if (port.length > 0) {
    return `${proto}://${host}:${port}/play`;
  }
  return `${proto}://${host}/play`;
};

const handlePosition = (message: MessagePosition) => {
  console.log("handlePosition", message);
  assign("position", position("white", message.legalMoves));
};
const handleEngineMove = (message: MessageEngineMove) => {
  console.log("handleEngineMove", message);
  hitClock();
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

const handleIcoming = (onReady: () => void) => (event: MessageEvent) => {
  const message = JSON.parse(event.data);

  switch (message._tag) {
    case "Ready": {
      assign("started", true);
      onReady();
      return console.log("server ready");
    }
    case "Position":
      return handlePosition(message as MessagePosition);
    case "EngineMove":
      return handleEngineMove(message as MessageEngineMove);
    case "Outcome":
      return handleOutcome(message as MessageOutcome);
  }
};

const CONNECT_TIMEOUT = 2000;

export const connect = () =>
  new Promise<string>((resolve, reject) => {
    const timeoutError = setTimeout(
      () => reject("Timeout error"),
      CONNECT_TIMEOUT
    );
    socket = new WebSocket(socketURL());
    socket.addEventListener(
      "message",
      handleIcoming(() => {
        clearTimeout(timeoutError);
        const fen = get("gameConfig").position;
        if (socket && fen !== null) {
          socket.send(
            JSON.stringify({
              _tag: "Position",
              fen,
              white_time: get("gameConfig").white,
              black_time: get("gameConfig").black,
            })
          );
        }
        resolve("Ready");
      })
    );
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
