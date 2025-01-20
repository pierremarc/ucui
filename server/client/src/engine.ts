import { DIV, replaceNodeContent } from "./lib/html";
import { formatMove } from "./san";
import { get, subscribe } from "./store";

export const mountEngine = (root: HTMLElement) => {
  const engine = DIV("engine");

  const setEngine = replaceNodeContent(engine);

  subscribe("engine")(() => {
    const state = get("engine");
    switch (state._tag) {
      case "idle":
        return setEngine("·");
      case "compute":
        return setEngine(DIV("compute"));
      case "move":
        return setEngine(formatMove(state.move, state.legals));
    }
  });

  root.append(engine);
};
