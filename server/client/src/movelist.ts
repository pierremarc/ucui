import { events } from "./lib/dom";
import { DIV, replaceNodeContent, SPAN } from "./lib/html";
import { formatMove } from "./san";
import { assign, get, subscribe } from "./store";

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

const makeMoves = () =>
  group(2, get("moveList")).map((g, i) => {
    const s0 = formatMove(g[0].move, g[0].legals, false).padEnd(8);
    if (g.length === 2) {
      const s1 = formatMove(g[1].move, g[1].legals, false);
      return DIV(
        "ply",
        SPAN("ord", `${i + 1}. `),
        SPAN("moves", `${s0} ${s1}`)
      );
    } else {
      return DIV("ply", SPAN("ord", `${i + 1}.  `), SPAN("moves", s0));
    }
  });

export const mountMoveList = (root: HTMLElement) => {
  const moves = DIV("moves", ...makeMoves());
  const back = get("started")
    ? events(DIV("back-button", "back to game"), (add) =>
        add("click", () => assign("screen", "game"))
      )
    : events(DIV("back-button", "back home"), (add) =>
        add("click", () => assign("screen", "home"))
      );

  root.append(
    DIV("movelist", moves, DIV("outcome", get("outcome") ?? "..."), back)
  );
  const replaceMoves = replaceNodeContent(moves);
  const sub = subscribe("moveList");
  sub(() => {
    replaceMoves(...makeMoves());
  });
};
