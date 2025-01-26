import { events } from "../lib/dom";
import { SPAN, DIV, replaceNodeContent } from "../lib/html";
import { MoveHist, savedGame } from "../lib/ucui/types";
import { group, setClipboard } from "../lib/util";
import { formatMove } from "./san";
import { assign, dispatch, get, getTurn, subscribe } from "./store";

const pendingMove = { _tag: "pending" as const };
type PendingMove = typeof pendingMove;

type HistOrPending = MoveHist | PendingMove;

export const pgn = (moves: MoveHist[]) =>
  group(2, moves)
    .map((g, i) => {
      const m0 = g[0];
      const m1 = g[1];
      if (m0 && m1) {
        return `${i + 1}. ${formatMove(m0.move, m0.legals, false)} ${formatMove(
          m1.move,
          m1.legals,
          false
        )} `;
      } else if (m0) {
        return `${i + 1}. ${formatMove(m0.move, m0.legals, false)} `;
      }
      return "";
    })
    .join("\n");

const moveList = (): HistOrPending[] =>
  getTurn() === get("gameConfig").engineColor
    ? (get("moveList") as HistOrPending[]).concat(pendingMove)
    : get("moveList");

const renderMoveHist = (mh: MoveHist) =>
  SPAN("move", formatMove(mh.move, mh.legals, true), "  ");

const renderPending = () => DIV("pending");

const renderMove = (m: HistOrPending) => {
  switch (m._tag) {
    case "pending":
      return renderPending();
    case "hist":
      return renderMoveHist(m);
  }
};

const makeMoves = () =>
  group(2, moveList()).map((g, i) => {
    const m0 = g[0];
    const m1 = g[1];
    if (m0 && m1) {
      return DIV(
        "ply",
        SPAN("ord", `${i + 1}. `),
        SPAN("moves", renderMove(m0), renderMove(m1))
      );
    } else if (m0) {
      return DIV(
        "ply",
        SPAN("ord", `${i + 1}.  `),
        SPAN("moves", renderMove(m0))
      );
    }
    return DIV("empty");
  });

const renderBack = () =>
  get("started")
    ? events(DIV("button", "Game"), (add) =>
        add("click", () => assign("screen", "game"))
      )
    : events(DIV("button", "Home"), (add) =>
        add("click", () => assign("screen", "home"))
      );

const renderCopyPgn = () =>
  events(DIV("button", "Copy PGN"), (add) =>
    add("click", () => setClipboard(pgn(get("moveList"))))
  );

const renderSaveGame = () =>
  events(DIV("button", "Save Game"), (add) =>
    add("click", () => {
      const hist = get("moveList");
      const config = get("gameConfig");
      const outcome = get("outcome");
      const timestamp = Date.now();
      dispatch("savedGames", (state) =>
        state.concat(savedGame(hist, config, outcome, timestamp))
      );
    })
  );

export const mountMoveList = (root: HTMLElement) => {
  const moves = DIV("moves", ...makeMoves());
  const back = DIV("back", renderBack());
  root.append(
    DIV(
      "movelist",
      moves,
      DIV("outcome", get("outcome") ?? "..."),
      DIV("actions", renderSaveGame(), renderCopyPgn(), back)
    )
  );
  const replaceMoves = replaceNodeContent(moves);
  const replaceBack = replaceNodeContent(back);
  const subList = subscribe("moveList");
  const subBack = subscribe("started");
  subList(() => {
    replaceMoves(...makeMoves());
  });
  subBack(() => {
    replaceBack(renderBack());
  });
};
