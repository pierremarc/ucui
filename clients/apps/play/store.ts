import {
  Screen,
  Color,
  otherColor,
  gameConfig,
  Input,
  inputNone,
  position,
  MoveHist,
  ClockState,
  clockInitial,
  Eco,
  defaultEngine,
  Nullable,
  SavedGame,
  FEN_INITIAL_POSITION,
} from "../lib/ucui/types";

import { startingLegalMoves } from "./data";

export const getTurn = (): Color =>
  get("moveList").length % 2 === 0 ? "white" : "black";

export const getPlayerColor = (): Color =>
  otherColor(get("gameConfig").engineColor);

export const defaultGameConfig = () =>
  gameConfig(10 * 60 * 1000, 60 * 1000, "black");
export const defaultInput = (): Input => inputNone();
export const defaultPosition = () =>
  position(startingLegalMoves, FEN_INITIAL_POSITION);
export const defaultScreen = (): Screen => "home";
export const defaultMoveList = (): MoveHist[] => [];
export const defaultClock = (): ClockState => clockInitial();
export const defaultEcoList = (): Eco[] => [];

let state = {
  screen: defaultScreen(),
  moveList: defaultMoveList(),
  clock: defaultClock(),
  position: defaultPosition(),
  input: defaultInput(),
  started: false,
  engine: defaultEngine(),
  engineName: "??",
  lockScreen: false,
  outcome: null as Nullable<string>,
  gameConfig: defaultGameConfig(),
  ecoResult: defaultEcoList(),
  savedGames: [] as SavedGame[],
};

export type State = typeof state;
export type StateKey = keyof State;

const storedKeys: StateKey[] = ["gameConfig", "savedGames"];

const loadFromStorage = () =>
  storedKeys.map(<K extends StateKey>(key: K) => {
    const item = localStorage.getItem(key);
    if (item !== null) {
      state[key] = JSON.parse(item) as State[K];
    }
  });

loadFromStorage();

let subs: [StateKey, (key: StateKey) => void][] = [];

export const dispatch = <K extends StateKey>(
  key: K,
  f: (val: State[K]) => State[K]
) => {
  let val = get(key);
  state[key] = f(val);
  if (storedKeys.includes(key)) {
    localStorage.setItem(key, JSON.stringify(state[key]));
  }
  if (key !== "clock") {
    console.groupCollapsed(key);
    console.debug("from", val);
    console.debug("to", state[key]);
    console.groupEnd();
  }
  subs.filter(([k, _]) => k == key).map(([_, cb]) => cb(key));
  return get(key);
};

export const assign = <K extends StateKey>(key: K, val: State[K]) =>
  dispatch(key, () => val);

export const get = <K extends StateKey>(key: K): State[K] =>
  JSON.parse(JSON.stringify(state[key]));

export const allKeys = () => Object.keys(state) as StateKey[];

export const subscribe =
  (...keys: StateKey[]) =>
  (callback: (key: StateKey) => void) =>
    (subs = subs.concat(keys.map((k) => [k, callback])));

export const clearSubscriptions = (filter: (k: StateKey) => boolean) =>
  (subs = subs.filter(([k, _]) => filter(k)));
