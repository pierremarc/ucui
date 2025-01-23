import { startingLegalMoves } from "./data";

export type Screen = "home" | "game" | "movelist" | "config";

export type Role = "Pawn" | "Knight" | "Bishop" | "Rook" | "Queen" | "King";

export type File = "A" | "B" | "C" | "D" | "E" | "F" | "G" | "H";
export type Rank = "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8";
export type Square = `${File}${Rank}`;

export const file = (s: Square) => s[0] as File;
export const rank = (s: Square) => s[1] as Rank;

export type Nullable<T> = T | null;

export type MoveNormal = {
  readonly _tag: "Normal";
  role: Role;
  from: Square;
  capture: Nullable<Role>;
  to: Square;
  promotion: Nullable<Role>;
};

export const moveNormal = (
  role: Role,
  from: Square,
  to: Square,
  capture: Nullable<Role>,
  promotion: Nullable<Role>
): MoveNormal => ({
  _tag: "Normal",
  role,
  from,
  to,
  capture: capture,
  promotion: promotion,
});

export type MoveEnPassant = {
  readonly _tag: "EnPassant";
  from: Square;
  to: Square;
};
export const moveEnPassant = (from: Square, to: Square): MoveEnPassant => ({
  _tag: "EnPassant",
  from,
  to,
});

export type MoveCastle = {
  readonly _tag: "Castle";
  king: Square;
  rook: Square;
};

export const MoveCastle = (king: Square, rook: Square): MoveCastle => ({
  _tag: "Castle",
  king,
  rook,
});

export type Move = MoveNormal | MoveCastle | MoveEnPassant;

export const getMoveRole = (move: Move): Role => {
  switch (move._tag) {
    case "Castle":
      return "King";
    case "EnPassant":
      return "Pawn";
    case "Normal":
      return move.role;
  }
};

export type Color = "black" | "white";

export const otherColor = (color: Color): Color =>
  color === "black" ? "white" : "black";

export type ClockInitial = { readonly _tag: "initial" };

export const clockInitial = (): ClockInitial => ({ _tag: "initial" });

export type ClockRunning = {
  readonly _tag: "running";
  turn: Color;
  start_time: number;
  remaining_white: number;
  remaining_black: number;
};

export const clockRunning = (
  turn: Color,
  start_time: number,
  remaining_white: number,
  remaining_black: number
): ClockRunning => ({
  _tag: "running",
  turn,
  start_time,
  remaining_white,
  remaining_black,
});

export type ClockFlag = {
  readonly _tag: "flag";
  color: Color; // fallen color
  other: number; // other's time
};

export const clockFlag = (color: Color, other: number): ClockFlag => ({
  _tag: "flag",
  color,
  other,
});

export type ClockState = ClockInitial | ClockRunning | ClockFlag;

export type Position = {
  turn: Color;
  legalMoves: Move[];
};

export const position = (turn: Color, legalMoves: Move[]): Position => ({
  legalMoves,
  turn,
});

export type InputNone = {
  readonly _tag: "none";
};

export const inputNone = (): InputNone => ({
  _tag: "none",
});

export type InputRole = {
  readonly _tag: "role";
  role: Role;
};

export const inputRole = (role: Role): InputRole => ({
  _tag: "role",
  role,
});

export type InputMove = {
  readonly _tag: "move";
  move: Move;
};

export const inputMove = (move: Move): InputMove => ({
  _tag: "move",
  move,
});

export type Input = InputNone | InputRole | InputMove;

export const getInputRole = (input: Input): Nullable<Role> => {
  switch (input._tag) {
    case "none":
      return null;
    case "role":
      return input.role;
    case "move":
      return getMoveRole(input.move);
  }
};

type EngineIdle = { readonly _tag: "idle" };
export const engineIdle = (): EngineIdle => ({ _tag: "idle" });
type EngineComputing = { readonly _tag: "compute" };
export const engineCompute = (): EngineComputing => ({ _tag: "compute" });
type EngineMove = {
  readonly _tag: "move";
  move: Move;
  legals: Move[];
  status: string;
};
export const engineMove = (
  move: Move,
  legals: Move[],
  status = ""
): EngineMove => ({
  _tag: "move",
  move,
  legals,
  status,
});

export type EngineState = EngineIdle | EngineComputing | EngineMove;
export const defaultEngine = (): EngineState => engineIdle();

export type MoveHist = {
  move: Move;
  legals: Move[];
};
export const moveHist = (move: Move, legals: Move[]): MoveHist => ({
  move,
  legals,
});

type GameConfig = {
  black: number;
  white: number;
  position: Nullable<string>;
};

export const gameConfig = (
  white: number,
  black: number,
  position = null as Nullable<string>
): GameConfig => ({ black, white, position });

export const defaultGameConfig = () => gameConfig(10 * 60 * 1000, 60 * 1000);
export const defaultInput = (): Input => inputNone();
export const defaultPosition = () => position("white", startingLegalMoves);
export const defaultScreen = (): Screen => "home";
export const defaultMoveList = (): MoveHist[] => [];
export const defaultClock = (): ClockState => clockInitial();

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
};

export type State = typeof state;
export type StateKey = keyof State;

let subs: [StateKey, (key: StateKey) => void][] = [];

export const dispatch = <K extends StateKey>(
  key: K,
  f: (val: State[K]) => State[K]
) => {
  let val = get(key);
  state[key] = f(val);
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
