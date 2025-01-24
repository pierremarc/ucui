import { startGame } from "./game";
import { emptyElement, events } from "./lib/dom";
import { DIV, INPUT } from "./lib/html";
import { connect } from "./play";
import { assign, dispatch, Eco, get, subscribe } from "./store";

const url = (term: string) => {
  const host = document.location.hostname;
  const proto = document.location.protocol;
  const port =
    document.location.port.length > 0 && document.location.port !== "8000"
      ? "8000"
      : document.location.port;
  if (port.length > 0) {
    return `${proto}//${host}:${port}/eco?term=${encodeURIComponent(term)}`;
  }
  return `${proto}//${host}/eco?term=${encodeURIComponent(term)}`;
};

const lookupTerm = (term: string) =>
  fetch(url(term), {
    mode: "cors",
    cache: "default",
    redirect: "follow",
    credentials: "same-origin",
  })
    .then((response) => {
      if (response.ok) {
        return response.json();
      }
      throw response;
    })
    .then((result: Eco[]) => assign("ecoResult", result))
    .catch((err) => console.error("failed to get eco", err));

const startGameFromEco = (eco: Eco) => {
  dispatch("gameConfig", (state) => ({ ...state, position: eco.fen }));
  connect()
    .then(() => {
      startGame(
        eco.moves.map((move) => ({
          legals: [],
          move,
        }))
      );
      assign("screen", "movelist");
    })
    .catch((err) => console.error("Connectin failed", err));
};
const renderItem = (eco: Eco) =>
  DIV(
    "item",
    DIV("name", eco.name),
    DIV("code", eco.code),
    events(DIV("play", "▶"), (add) => add("click", () => startGameFromEco(eco)))
  );

const renderItems = (root: HTMLElement) => {
  emptyElement(root);
  root.append(...get("ecoResult").map(renderItem));
};

export const renderEco = () => {
  const ecolist = DIV("listing");
  const input = INPUT("i", "text");
  const lookup = DIV(
    "lookup",
    input,
    events(DIV("go-button", "search"), (add) =>
      add("click", () => lookupTerm(input.value))
    )
  );

  subscribe("ecoResult")(() => renderItems(ecolist));

  return DIV(
    "eco",
    DIV("help", "Lookup an opening  by name and start a game from there."),
    lookup,
    ecolist
  );
};
