import { events } from "../lib/dom";
import { DIV, H2, replaceNodeContent } from "../lib/html";
import { fromNullable, map, none, some } from "../lib/option";
import { SavedGame } from "../lib/ucui/types";
import { startGame } from "./game";
import { pgn } from "./movelist";
import { connect, disconnect } from "./play";
import { assign, dispatch, get, subscribe } from "./store";

const formatTime = (n: number) => {
  const d = new Date(n);
  return `${d.toLocaleDateString()} - ${d.toLocaleTimeString()}`;
};

const renderOutcome = map((o: string) => `  (${o}) `);

const renderMoves = (game: SavedGame) => DIV("moves", pgn(game.hist));

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
      events(DIV("play", "▶"), (add) =>
        add("click", () => startGameFromHistItem(game))
      )
    )
  );

const renderGame = (game: SavedGame) =>
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
  );

const renderHistory = () => get("savedGames").map(renderGame).reverse();

export const mountHistory = (root: HTMLElement) => {
  const games = DIV("listing", ...renderHistory());
  root.append(DIV("history", H2("title", "Saved games"), games));
  const replace = replaceNodeContent(games);
  const sub = subscribe("savedGames");
  sub(() => {
    replace(...renderHistory());
  });
};

const withoutOutcome = (game: SavedGame, node: HTMLElement) =>
  game.outcome === null ? some(node) : none;

const startGameFromHistItem = (game: SavedGame) => {
  console.log("sart from hist", game);
  disconnect();
  assign("gameConfig", game.config);
  connect()
    .then(() => {
      startGame(game.hist);
      assign("screen", "game");
    })
    .catch((err) => console.error("Connectin failed", err));
};
