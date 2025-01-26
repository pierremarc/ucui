import { attrs } from "../lib/dom";
import { AUDIO } from "../lib/html";
import soundUrl from "../assets/chess.ogg";

const audio = attrs(AUDIO("-", soundUrl), (set) => set("preload", "auto"));
document.body.append(audio);
// const initSound = () => {

//     document
// }

export const playSound = () => {
  audio.play();
};
