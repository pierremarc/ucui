import { events, removeElement } from "../lib/dom";
import { addClass, DIV, replaceNodeContent } from "../lib/html";
import { fromNullable, map } from "../lib/option";
import { Color, Nullable, Role } from "../lib/ucui/types";
import { fenToRanks, OccupProc } from "./fen";
import "./style.css";

let socket: Nullable<WebSocket> = null;
const CONNECT_TIMEOUT = 4000;

type MessageUpdate = { readonly _tag: "Update"; games: [string, string][] };
type MessageInit = { readonly _tag: "Init"; games: [string, string][] };
type Message = MessageInit | MessageUpdate;

const socketURL = () => {
  const host = document.location.hostname;
  const proto = host !== "localhost" && host !== "127.0.0.1" ? "wss" : "ws";
  const port =
    document.location.port.length > 0 && document.location.port !== "8000"
      ? "8000"
      : document.location.port;
  const url =
    port.length > 0
      ? `${proto}://${host}:${port}/games`
      : `${proto}://${host}/games`;

  return url;
};

const connect = () =>
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
      console.warn("Connection closed");
    });
  });

const handleIcoming = (event: MessageEvent) => {
  const message = JSON.parse(event.data) as Message;

  switch (message._tag) {
    case "Init":
      return handleInit(message);
    case "Update":
      return handleUpdate(message);
  }
};

type Status = "new" | "ongoing" | "end";

type Game = {
  key: string;
  fen: string;
  status: Status;
};

const recToGame = (g: [string, string], status: Status): Game => ({
  status,
  key: mkId(g[0]),
  fen: g[1],
});

let state: Game[] = [];

const findGame = (id: string) => state.find((g) => g.key === id);

const removeGameFromState = (id: string) =>
  (state = state.filter((g) => g.key !== id));

const mkId = (s: string) => "id-" + s;

const handleInit = (message: MessageInit) => {
  console.log("Init", message.games);
  state = message.games.map((g) => recToGame(g, "new"));
  updateView();
};

const handleUpdate = (message: MessageUpdate) => {
  const incomingKeys = message.games.map(([key, _]) => mkId(key));

  const games = message.games.map(([key, fen]) => {
    const game = findGame(mkId(key));
    if (game) {
      return recToGame([key, fen], "ongoing");
    }
    return recToGame([key, fen], "new");
  });

  const oldGames = state
    .filter((g) => !incomingKeys.includes(g.key))
    .map<Game>((g) => ({ ...g, status: "end" }));
  state = games.concat(oldGames);

  updateView();
};

let rootElement: Nullable<HTMLElement> = null;

const roleLetter = (role: Role, color: Color) => {
  switch (role) {
    case "Pawn":
      return color === "black" ? "♟" : "♙";
    case "Rook":
      return color === "black" ? "♜" : "♖";
    case "Knight":
      return color === "black" ? "♞" : "♘";
    case "Bishop":
      return color === "black" ? "♝" : "♗";
    case "Queen":
      return color === "black" ? "♛" : "♕";
    case "King":
      return color === "black" ? "♚" : "♔";
  }
};

const makeOccup: OccupProc<HTMLElement> = (square, occup) => {
  if (occup === null) {
    return DIV(`square empty ${square}`, ".");
  }
  return DIV(`square ${square}`, roleLetter(occup.role, occup.color));
};

const makeBoard = (fen: string) =>
  DIV(
    "board",
    ...fenToRanks(fen, makeOccup).map((squares) => DIV("rank", ...squares))
  );

const end = addClass("end");

const updateView = () => {
  // const existingGame = state.filter(g => g.status === 'ongoing')
  rootElement?.querySelectorAll(".game").forEach((elem) => {
    const id = elem.id;
    const game = findGame(id);

    if (game) {
      replaceNodeContent(elem as HTMLElement)(makeBoard(game.fen));
      if (game.status === "end") {
        end(elem as HTMLElement);
        const rem = elem.querySelector(".remove");
        if (rem === null) {
          elem.append(
            events(DIV("remove", "remove"), (add) =>
              add("click", () =>
                setTimeout(() => {
                  removeGameFromState(id);
                  removeElement(elem);
                }, 120)
              )
            )
          );
        }
      }
    }
  });

  state
    .filter((g) => g.status === "new")
    .map((g) => {
      const game = DIV("game", makeBoard(g.fen));
      game.id = g.key;
      rootElement?.append(game);
    });
};

const main = (root: HTMLElement) => {
  const status = DIV("status", "Waiting for server...");
  const games = DIV("game-list");
  root.append(status, games);
  rootElement = games;

  connect()
    .then(() => {
      replaceNodeContent(status)("Connected");
      updateView();
    })
    .catch((err) => replaceNodeContent(status)(`Failed to connect: ${err}`));
};

map(main)(fromNullable(document.querySelector<HTMLDivElement>("#app")));
