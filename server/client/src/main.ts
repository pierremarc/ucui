import { fromNullable, map } from "./lib/option";
import "./style.css";
import { clearSubscriptions, get, subscribe } from "./store";
import { mountHome } from "./home";
import { mountGame } from "./game";
import { emptyElement } from "./lib/dom";
import { screenLocker } from "./lock-screen";
import { mountMoveList } from "./movelist";

const main = (root: HTMLElement) => {
  screenLocker();
  mountHome(root);

  subscribe("screen")(() => {
    clearSubscriptions((k) => k === "screen");
    emptyElement(root);
    switch (get("screen")) {
      case "home":
        return mountHome(root);
      case "game":
        return mountGame(root);
      case "movelist":
        return mountMoveList(root);
    }
  });

  // const mount = appendNode(root);
  // mount(events(DIV("", "hit"), (add) => add("click", hit)));

  // setTimeout(() => {
  //   assign("input", position(startingLegalMoves));
  // }, 1000);
};

map(main)(fromNullable(document.querySelector<HTMLDivElement>("#app")));
