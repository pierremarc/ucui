import { events, emptyElement } from "../lib/dom";
import { DIV, replaceNodeContent, INPUT, AcNode } from "../lib/html";
import { Eco, moveHist } from "../lib/ucui/types";
import { iife } from "../lib/util";
import { startGame } from "./game";
import { connect } from "./play";
import { assign, dispatch, get, subscribe } from "./store";
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
  }).then((response) => {
    if (response.ok) {
      return response.json();
    }
    throw response;
  });
};

const lookupTerm = (term: string, setList: (...values: AcNode[]) => void) =>
  fetchJSON("/eco", { term })
    .then((result: Eco[]) =>
      assign(
        "ecoResult",
        result.sort((a, b) => {
          const code = a.code.localeCompare(b.code);
          return code === 0 ? a.moves.length - b.moves.length : code;
        })
      )
    )
    .catch(() =>
      setList(
        `Failed to get an openings list. The server might be down, please retry later.`
      )
    );

const startGameFromEco = (eco: Eco) => {
  dispatch("gameConfig", (state) => ({ ...state, position: eco.fen }));
  connect()
    .then(() => {
      startGame(eco.moves.map((move) => moveHist(move, [], eco.fen)));
      assign("screen", "game");
    })
    .catch((err) => console.error("Connectin failed", err));
};

const renderItem = (eco: Eco) =>
  DIV(
    "item",
    DIV("names", DIV("code", eco.code), DIV("name", eco.name)),
    DIV(
      "actions",
      DIV("moves", eco.pgn),
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
  const setList = replaceNodeContent(ecolist);

  const handlerSearch = () => {
    setList(DIV("loader", "searching..."));
    const term = input.value;
    if (term.length > 0) {
      lookupTerm(term, setList).then(() => {
        input.blur();
        input.value = "";
      });
    }
  };
  const input = events(INPUT("i", "search"), (add) =>
    add("change", handlerSearch)
  );

  const lookup = DIV(
    "lookup",
    input,
    events(DIV("go-button", "search"), (add) => add("click", handlerSearch))
  );

  subscribe("ecoResult")(() => renderItems(ecolist));

  return DIV(
    "eco",
    DIV("help", "Lookup an opening  by name."),
    lookup,
    ecolist
  );
};
