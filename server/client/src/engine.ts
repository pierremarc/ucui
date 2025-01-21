import { events } from "./lib/dom";
import { DIV, replaceNodeContent } from "./lib/html";
import { formatMove } from "./san";
import { assign, get, subscribe } from "./store";

const render = (root: HTMLElement) => {
  const setEngine = replaceNodeContent(root);
  const state = get("engine");
  switch (state._tag) {
    case "idle":
      return setEngine("·");
    case "compute":
      return setEngine(DIV("compute"));
    case "move":
      return setEngine(formatMove(state.move, state.legals));
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
