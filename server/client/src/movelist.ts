import { events } from "./lib/dom";
import { DIV, SPAN } from "./lib/html";
import { formatMove } from "./san";
import { assign, get } from "./store";

const group = <T>(n: number, as: T[]): T[][] => {
  const result: T[][] = [[]];
  for (let i = 0; i < as.length; i++) {
    let index = Math.floor(i / n);
    if (index === result.length) {
      result.push([]);
    }
    result[index].push(as[i]);
  }
  return result;
};

export const mountMoveList = (root: HTMLElement) => {
  const pairs = group(2, get("moveList")).map((g, i) => {
    const s0 = formatMove(g[0].move, g[0].legals, false);
    if (g.length === 2) {
      const s1 = formatMove(g[1].move, g[1].legals, false);
      return DIV(
        "ply",
        SPAN("ord", `${i + 1}.  `),
        SPAN("moves", SPAN("white", s0), SPAN("black", s1))
      );
    } else {
      return DIV(
        "ply",
        SPAN("ord", `${i + 1}.  `),
        SPAN("moves", SPAN("white", s0))
      );
    }
  });

  const back = get("started")
    ? events(DIV("back-button", "back to game"), (add) =>
        add("click", () => assign("screen", "game"))
      )
    : events(DIV("back-button", "back home"), (add) =>
        add("click", () => assign("screen", "home"))
      );

  root.append(
    DIV("movelist", ...pairs, DIV("outcome", get("outcome") ?? "..."), back)
  );
};
