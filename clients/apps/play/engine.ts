import { events } from "../lib/dom";
import { replaceNodeContent, SPAN, DIV } from "../lib/html";
import { formatMove } from "./san";
import { assign, get, getPlayerColor, getTurn, subscribe } from "./store";

const render = (engineInfo: HTMLElement, engineState: HTMLElement) => {
  const state = get("engine");
  const setEngine = replaceNodeContent(engineState);
  const setEngineInfo = replaceNodeContent(engineInfo);
  setEngineInfo(SPAN("name", get("engineName")));
  switch (state._tag) {
    case "idle": {
      const turn = getTurn();
      if (turn == getPlayerColor()) {
        return setEngine(DIV("idle", `Your turn to play ${turn}`));
      }
      return setEngine(DIV("idle", `Engine will to play ${turn}`));
    }
    case "compute":
      return setEngine(DIV("compute"));
    case "move":
      return setEngine(formatMove(state.move, state.legals) + state.status);
  }
};

export const mountEngine = (root: HTMLElement) => {
  const engineInfo = DIV("info");
  const engineState = DIV("state");
  const engine = events(DIV("engine", engineInfo, engineState), (add) =>
    add("click", () => assign("screen", "movelist"))
  );

  subscribe("engine", "engineName")(() => render(engineInfo, engineState));
  render(engineInfo, engineState);
  root.append(engine);
};
