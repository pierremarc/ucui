import { DIV } from "../lib/html";
import { fromNullable, map } from "../lib/option";
import "./style.css";

const main = (root: HTMLElement) => {
  root.append(DIV("tada", "TADA"));
};

map(main)(fromNullable(document.querySelector<HTMLDivElement>("#app")));
