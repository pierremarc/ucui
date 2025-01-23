import { events } from "./lib/dom";
import { DIV, replaceNodeContent, SPAN } from "./lib/html";
import { formatMove } from "./san";
import { assign, get, subscribe } from "./store";

const render = (root: HTMLElement) => {
  const state = get("engine");
  const engineInfo = DIV("info", SPAN("name", get("engineName")));
  const engineState = DIV("state");
  const setEngine = replaceNodeContent(engineState);
  root.append(engineInfo, engineState);
  switch (state._tag) {
    case "idle":
      return setEngine("·");
    case "compute":
      return setEngine(DIV("compute"));
    case "move":
      return setEngine(formatMove(state.move, state.legals) + state.status);
  }
};

export const mountEngine = (root: HTMLElement) => {
  const engine = events(DIV("engine"), (add) =>
    add("click", () => assign("screen", "movelist"))
  );
  render(engine);
  subscribe("engine")(() => render(engine));

  root.append(engine);
};
