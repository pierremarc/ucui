import { events } from "../lib/dom";
import { replaceNodeContent, SPAN, DIV } from "../lib/html";
import { EngineScore } from "../lib/ucui/types";
import { formatMove } from "./san";
import { assign, get, getPlayerColor, getTurn, subscribe } from "./store";

const renderScore = (score: EngineScore) => {
  switch (score._tag) {
    case "None":
      return DIV(" score-none", "??");
    case "CentiPawns":
      return DIV(" score-cp", (score.score / 100).toFixed(1));
    case "Mate":
      return DIV(
        " score-mate ",
        score.moves < 0
          ? `Engine fears a mate in ${Math.abs(score.moves)}`
          : `Engine sees you  mate in ${score.moves}`
      );
  }
};

const render = (
  engineInfo: HTMLElement,
  engineScore: HTMLElement,
  engineState: HTMLElement
) => {
  const state = get("engine");
  const setEngine = replaceNodeContent(engineState);
  const setEngineScore = replaceNodeContent(engineScore);
  const setEngineInfo = replaceNodeContent(engineInfo);
  setEngineInfo(SPAN("name", get("engineName")));

  switch (state._tag) {
    case "idle": {
      const turn = getTurn();
      if (turn == getPlayerColor()) {
        return setEngine(DIV("idle", `Your turn to play ${turn}`));
      }
      return setEngine(DIV("idle", `Engine to play ${turn}`));
    }
    case "compute":
      return setEngine(DIV("compute"));
    case "move": {
      setEngineScore(renderScore(state.score));
      setEngine(formatMove(state.move, state.legals) + state.status);
      return;
    }
  }
};

export const mountEngine = (root: HTMLElement) => {
  const engineInfo = DIV("info");
  const engineScore = DIV("score");
  const engineState = DIV("state");

  const toListButton = events(DIV("to-list to-button", "↪"), (add) =>
    add("click", () => assign("screen", "movelist"))
  );

  render(engineInfo, engineScore, engineState);
  const engine = DIV(
    "engine",
    engineInfo,
    engineScore,
    engineState,
    toListButton
  );

  root.append(engine);

  subscribe(
    "engine",
    "engineName"
  )(() => render(engineInfo, engineScore, engineState));
};
