import { events } from "../lib/dom";
import { SPAN, DIV, replaceNodeContent, H2 } from "../lib/html";
import { MoveHist, Nullable, savedGame } from "../lib/ucui/types";
import { group, setClipboard } from "../lib/util";
import { defaultFormat, defaultFormatSymbol, formatMove } from "./san";
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
        return `${i + 1}. ${formatMove(
          m0.move,
          m0.legals,
          defaultFormat
        )} ${formatMove(m1.move, m1.legals, defaultFormat)} `;
      } else if (m0) {
        return `${i + 1}. ${formatMove(m0.move, m0.legals, defaultFormat)} `;
      }
      return "";
    })
    .join("\n");

const moveList = (): HistOrPending[] =>
  getTurn() === get("gameConfig").engineColor
    ? (get("moveList") as HistOrPending[]).concat(pendingMove)
    : get("moveList");

const renderMoveHist = (mh: MoveHist) =>
  SPAN("move", formatMove(mh.move, mh.legals, defaultFormatSymbol), "  ");

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

// const renderBack = () =>
//   get("started")
//     ? events(DIV("button", "Game"), (add) =>
//         add("click", () => assign("screen", "game"))
//       )
//     : events(DIV("button", "Home"), (add) =>
//         add("click", () => assign("screen", "home"))
//       );

const renderCopyPgn = () =>
  events(DIV("button", "Copy PGN"), (add) =>
    add("click", () => setClipboard(pgn(get("moveList"))))
  );

let lastSavedGame: Nullable<number> = null;

const renderSaveGame = () =>
  lastSavedGame !== null
    ? DIV("button disabled", "Game saved")
    : events(DIV("button", "Save game"), (add) =>
        add("click", () => {
          lastSavedGame = window.setTimeout(() => {
            lastSavedGame = null;
            assign("started", get("started")); // poor man reload
          }, 12 * 1000);
          const hist = get("moveList");
          const config = get("gameConfig");
          const outcome = get("outcome");
          const timestamp = Date.now();
          dispatch("savedGames", (state) =>
            state.concat(savedGame(hist, config, outcome, timestamp))
          );
        })
      );

const renderActions = () => [renderSaveGame(), renderCopyPgn()];

const renderOutcome = () => get("outcome") ?? "...";

const header = () =>
  DIV(
    "header",
    H2("title", "Moves"),
    events(DIV("to-game  to-button", "↩"), (add) =>
      add("click", () => assign("screen", "game"))
    )
  );

export const mountMoveList = (root: HTMLElement) => {
  const moves = DIV("moves", ...makeMoves());
  const actions = DIV("actions", ...renderActions());
  const outcome = DIV("outcome", renderOutcome());
  root.append(
    DIV("movelist", header(), DIV("listing", moves, outcome), actions)
  );

  const replaceMoves = replaceNodeContent(moves);
  const replaceOutcome = replaceNodeContent(outcome);
  const replaceActions = replaceNodeContent(actions);
  const subList = subscribe("moveList");
  const subAction = subscribe("started", "savedGames");
  const subOuctome = subscribe("outcome");
  subList(() => {
    replaceMoves(...makeMoves());
  });
  subAction(() => {
    replaceActions(...renderActions());
  });
  subOuctome(() => {
    replaceOutcome(renderOutcome());
  });
};
