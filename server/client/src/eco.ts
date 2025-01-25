import { startGame } from "./game";
import { emptyElement, events } from "./lib/dom";
import { DIV, INPUT, replaceNodeContent, SPAN } from "./lib/html";
import { iife } from "./lib/util";
import { pgn } from "./movelist";
import { connect } from "./play";
import { assign, dispatch, Eco, get, Move, moveHist, subscribe } from "./store";
import { UrlQuery, withQueryString } from "./util";

const fetchJSON = (endpoint: string, query: UrlQuery) => {
  const host = document.location.hostname;
  const proto = document.location.protocol;
  const port =
    document.location.port.length > 0 && document.location.port !== "8000"
      ? "8000"
      : document.location.port;

  const url = iife(() => {
    if (port.length > 0) {
      return withQueryString(`${proto}//${host}:${port}${endpoint}`, query);
    }
    return withQueryString(`${proto}//${host}${endpoint}`, query);
  });
  return fetch(url, {
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
    .catch((err) => console.error("failed to get eco", err));
};

const lookupTerm = (term: string) =>
  fetchJSON("/eco", { term }).then((result: Eco[]) =>
    assign(
      "ecoResult",
      result.sort((a, b) => a.code.localeCompare(b.code))
    )
  );

const startGameFromEco = (eco: Eco) => {
  dispatch("gameConfig", (state) => ({ ...state, position: eco.fen }));
  connect()
    .then(() => {
      startGame(eco.moves.map((move) => moveHist(move, [])));
      assign("screen", "movelist");
    })
    .catch((err) => console.error("Connectin failed", err));
};

const renderMoves = ({ fen, moves }: Eco) => {
  const inner = DIV(
    "moves",
    events(SPAN("load_moves", "🛈"), (add) =>
      add("click", () =>
        fetchJSON("/legals", { fen }).then((legals: Move[]) =>
          replaceNodeContent(inner)(pgn(moves.map((m) => moveHist(m, legals))))
        )
      )
    )
  );

  return inner;
};

const renderItem = (eco: Eco) =>
  DIV(
    "item",
    DIV("names", DIV("code", eco.code), DIV("name", eco.name)),
    DIV(
      "actions",
      renderMoves(eco),
      events(DIV("play", "▶"), (add) =>
        add("click", () => startGameFromEco(eco))
      )
    )
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
