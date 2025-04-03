import { attrs, emptyElement, events, removeElement } from "../lib/dom";
import { DIV, H2, replaceNodeContent } from "../lib/html";
import { fromNullable, map, none, some } from "../lib/option";
import { fenToRanks, OccupProc } from "../lib/ucui/fen";
import { Color, gameConfig, Role, SavedGame } from "../lib/ucui/types";
import { group, letMap } from "../lib/util";
import { startGameWithMoves } from "./game";
import { connect, disconnect } from "./play";
import { formatMove, defaultFormat } from "./san";
import { assign, dispatch, get, subscribe } from "./store";

const formatTime = (n: number) => {
  const d = new Date(n);
  return `${d.toLocaleDateString()} - ${d.toLocaleTimeString()}`;
};

const renderOutcome = map((o: string) => `  (${o}) `);

const playSymbol = "▶";
("plus");

const pgn = (game: SavedGame) =>
  group(2, game.hist)
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

const renderPGNPlay = (game: SavedGame) =>
  group(2, game.hist).map((g, i) => {
    const m0 = g[0];
    const m1 = g[1];
    const baseIndex = i * 2;
    if (m0) {
      const m0Play = events(
        DIV("m0", formatMove(m0.move, m0.legals, defaultFormat)),
        (add) => add("click", () => selectMove(game, baseIndex))
      );
      if (m1) {
        const m1Play = events(
          DIV("m1", formatMove(m1.move, m1.legals, defaultFormat)),
          (add) => add("click", () => selectMove(game, baseIndex + 1))
        );
        return DIV("ply", DIV("ord", `${i + 1}.`), m0Play, m1Play);
      }
      return DIV("ply", DIV("ord", `${i + 1}.`), m0Play);
    }
    return DIV("ply empty");
  });

const renderFENPlay = (game: SavedGame, startIndex: number) => {
  let moveIndex = startIndex;
  const updatable = DIV("updatable-board");

  const update = () => {
    const board = letMap(game.hist[moveIndex], ({ resultingFen: fen }) =>
      makeBoard(fen)
    );
    const render = map((board: HTMLElement) => {
      const actions = DIV(
        "actions",
        events(DIV("cancel-button", "back"), (add) =>
          add("click", () => selectGame(game))
        ),
        events(DIV("play-button", "start"), (add) =>
          add("click", () => startGameFromHistItem(game, moveIndex))
        )
      );
      const prev =
        moveIndex === 0
          ? DIV("prev-button disabled", "◁")
          : events(DIV("prev-button", "◁"), (add) =>
              add("click", () => {
                moveIndex -= 1;
                update();
              })
            );

      const next =
        moveIndex === game.hist.length - 1
          ? DIV("next-button disabled", "▷")
          : events(DIV("next-button", "▷"), (add) =>
              add("click", () => {
                moveIndex += 1;
                update();
              })
            );

      const navGame = DIV("nav-game", prev, next);

      emptyElement(updatable);
      updatable.append(board, navGame, actions);
    });

    render(fromNullable(board));
  };

  update();
  return DIV("fen-play", updatable);
};

const renderMoves = (game: SavedGame) => DIV("moves", pgn(game));

const renderDelete = (game: SavedGame) =>
  events(DIV("delete", "delete"), (add) =>
    add("click", () =>
      dispatch("savedGames", (state) =>
        state.filter((g) => g.timestamp !== game.timestamp)
      )
    )
  );

const renderActions = (game: SavedGame) =>
  DIV(
    "actions",
    renderDelete(game),
    withoutOutcome(
      game,
      events(DIV("play", playSymbol), (add) =>
        add("click", () => selectGame(game))
      )
    )
  );

const mkid = (game: SavedGame) => `saved-${game.timestamp}`;

const renderGame = (game: SavedGame) =>
  attrs(
    DIV(
      "item",
      DIV(
        "names",
        DIV(
          "code",
          formatTime(game.timestamp),
          renderOutcome(fromNullable(game.outcome))
        )
      ),
      renderMoves(game),
      renderActions(game)
    ),
    (set) => set("id", mkid(game))
  );

const renderHistory = () => get("savedGames").map(renderGame).reverse();

const selectGame = (game: SavedGame) => {
  document
    .querySelectorAll(".pgn-play")
    .forEach((e) => removeElement(e as Element));
  letMap(document.getElementById(mkid(game)), (root) =>
    root.append(
      DIV(
        "pgn-play",
        DIV("help", "Tap a move to start from there."),
        ...renderPGNPlay(game)
      )
    )
  );
};

const selectMove = (game: SavedGame, moveIndex: number) => {
  document
    .querySelectorAll(".pgn-play")
    .forEach((e) => removeElement(e as Element));
  letMap(document.getElementById(mkid(game)), (root) =>
    root.append(DIV("pgn-play", renderFENPlay(game, moveIndex)))
  );
};

const header = () =>
  DIV(
    "header",
    H2("title", "Saved games"),
    events(DIV("to-home  to-button", "↩"), (add) =>
      add("click", () => assign("screen", "home"))
    )
  );

export const mountHistory = (root: HTMLElement) => {
  const games = DIV("listing", ...renderHistory());

  root.append(DIV("history", header(), games));
  const replace = replaceNodeContent(games);
  const sub = subscribe("savedGames");
  sub(() => {
    replace(...renderHistory());
  });
};

const withoutOutcome = (game: SavedGame, node: HTMLElement) =>
  game.outcome === null ? some(node) : none;

const startGameFromHistItem = (game: SavedGame, moveIndex: number) => {
  console.log("sart from hist", game);
  disconnect();
  letMap(game.hist[moveIndex], ({ resultingFen: fen }) => {
    const { white, black, engineColor } = game.config;
    assign("gameConfig", gameConfig(white, black, engineColor, fen));
    connect()
      .then(() => {
        startGameWithMoves(game.hist.slice(0, moveIndex + 1));
        assign("screen", "game");
      })
      .catch((err) => console.error("Connectin failed", err));
  });
};
