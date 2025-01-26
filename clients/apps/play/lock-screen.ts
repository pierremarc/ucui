import { Nullable } from "../lib/ucui/types";
import { allKeys, assign, get, subscribe } from "./store";

const hasLockAPI = "wakeLock" in navigator;

export const screenLocker = () => {
  if (hasLockAPI) {
    // we don't know much about the real screen state
    // so lets listen to whatever moves
    const sub = subscribe(...allKeys());

    let wsent: Nullable<WakeLockSentinel> = null;

    sub(() => {
      const started = get("started");
      const locked = get("lockScreen");
      if (started && !locked) {
        navigator.wakeLock
          .request("screen")
          .then((sentinel) => {
            wsent = sentinel;
            wsent.addEventListener("release", () => {
              console.log("Navigator released lock sentinel");
              assign("lockScreen", false);
            });
            assign("lockScreen", true);
          })
          .catch((err) => console.error("failed to lock screen", err));
      } else if (!started && locked && wsent !== null) {
        wsent
          .release()
          .then(() => {
            wsent = null;
            assign("lockScreen", false);
          })
          .catch((err) => console.error("failed to release screen", err));
      }
    });
  }
};
