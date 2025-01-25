import { AUDIO } from "./lib/html";
import soundUrl from "./chess.ogg";
import { attrs } from "./lib/dom";

const audio = attrs(AUDIO("-", soundUrl), (set) => set("preload", "auto"));
document.body.append(audio);
// const initSound = () => {

//     document
// }

export const playSound = () => {
  audio.play();
};
