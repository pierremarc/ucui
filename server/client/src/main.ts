import { fromNullable, map } from "./lib/option";
import "./style.css";
import { clearSubscriptions, get, StateKey, subscribe } from "./store";
import { mountHome } from "./home";
import { mountGame } from "./game";
import { emptyElement } from "./lib/dom";
import { screenLocker } from "./lock-screen";
import { mountMoveList } from "./movelist";

const fullscreen = (elem: HTMLElement) => (toggle: boolean) =>
  toggle
    ? elem
        .requestFullscreen()
        .then(() => console.log("enter fullscreen"))
        .catch((err) => console.error("failed to enter fullscreen", err))
    : document
        .exitFullscreen()
        .then(() => console.log("exir fullscreen"))
        .catch((err) => console.error("failed to exit fullscreen", err));

const main = (root: HTMLElement) => {
  screenLocker();
  mountHome(root);

  const toggleFullscreen = fullscreen(root);

  let keepSubs: StateKey[] = ["screen", "lockScreen"];

  subscribe("screen")(() => {
    clearSubscriptions((k) => keepSubs.includes(k));
    emptyElement(root);
    switch (get("screen")) {
      case "home": {
        toggleFullscreen(false);
        return mountHome(root);
      }
      case "game": {
        toggleFullscreen(true);
        return mountGame(root);
      }
      case "movelist": {
        toggleFullscreen(false);
        return mountMoveList(root);
      }
    }
  });

  // const mount = appendNode(root);
  // mount(events(DIV("", "hit"), (add) => add("click", hit)));

  // setTimeout(() => {
  //   assign("input", position(startingLegalMoves));
  // }, 1000);
};

map(main)(fromNullable(document.querySelector<HTMLDivElement>("#app")));