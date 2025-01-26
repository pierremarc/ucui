import { events } from "../lib/dom";
import { DETAILS, DIV, H2, replaceNodeContent } from "../lib/html";
import { fromNullable, map } from "../lib/option";
import { SavedGame } from "../lib/ucui/types";
import { pgn } from "./movelist";
import { dispatch, get, subscribe } from "./store";

const formatTime = (n: number) => {
  const d = new Date(n);
  return `${d.toLocaleDateString()} - ${d.toLocaleTimeString()}`;
};

const renderOutcome = map((o: string) => `  (${o}) `);

const renderMoves = (game: SavedGame) => DIV("pgn", pgn(game.hist));

const renderDelete = (game: SavedGame) =>
  events(DIV("delete", "delete"), (add) =>
    add("click", () =>
      dispatch("savedGames", (state) =>
        state.filter((g) => g.timestamp !== game.timestamp)
      )
    )
  );

const renderActions = (game: SavedGame) => DIV("actions", renderDelete(game));

const renderGame = (game: SavedGame) =>
  DIV(
    "game",
    DETAILS(
      "inner",
      H2(
        "date",
        formatTime(game.timestamp),
        renderOutcome(fromNullable(game.outcome))
      ),
      renderMoves(game),
      renderActions(game)
    )
  );

const renderHistory = () => get("savedGames").map(renderGame).reverse();

export const mountHistory = (root: HTMLElement) => {
  const games = DIV("game-list", ...renderHistory());
  root.append(DIV("history", games));
  const replace = replaceNodeContent(games);
  const sub = subscribe("savedGames");
  sub(() => {
    replace(...renderHistory());
  });
};
