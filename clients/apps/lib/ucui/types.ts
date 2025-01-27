/**
 * lint hint: this file should import nothing
 * and hold only types and contructors/accessors
 */

export type Screen = "home" | "game" | "movelist" | "config" | "history";

export type Role = "Pawn" | "Knight" | "Bishop" | "Rook" | "Queen" | "King";

export type SquareFile = "A" | "B" | "C" | "D" | "E" | "F" | "G" | "H";
export type SquareRank = "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8";
export type Square = `${SquareFile}${SquareRank}`;

export const squareFiles: SquareFile[] = [
  "A",
  "B",
  "C",
  "D",
  "E",
  "F",
  "G",
  "H",
];
export const squareRanks: SquareRank[] = [
  "1",
  "2",
  "3",
  "4",
  "5",
  "6",
  "7",
  "8",
];
export const getFile = (s: Square) => s[0] as SquareFile;
export const getRank = (s: Square) => s[1] as SquareRank;
export const makeSquare = (f: SquareFile, r: SquareRank) => (f + r) as Square;

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
  start_time: number;
  remaining_white: number;
  remaining_black: number;
};

export const clockRunning = (
  start_time: number,
  remaining_white: number,
  remaining_black: number
): ClockRunning => ({
  _tag: "running",
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
  // turn: Color;
  legalMoves: Move[];
  fen: string;
};

export const position = (legalMoves: Move[], fen: string): Position => ({
  legalMoves,
  fen,
});

export const FEN_INITIAL_POSITION =
  "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

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

export type EngineScoreNone = { readonly _tag: "None" };
export type EngineScoreMate = { readonly _tag: "Mate"; moves: number };
export type EngineScoreCentiPawns = {
  readonly _tag: "CentiPawns";
  score: number;
};
export type EngineScore =
  | EngineScoreNone
  | EngineScoreMate
  | EngineScoreCentiPawns;

export const engineScoreNone = (): EngineScore => ({ _tag: "None" });

type EngineMove = {
  readonly _tag: "move";
  move: Move;
  legals: Move[];
  status: string;
  score: EngineScore;
};
export const engineMove = (
  move: Move,
  legals: Move[],
  score: EngineScore,
  status: string
): EngineMove => ({
  _tag: "move",
  move,
  legals,
  score,
  status,
});

export type EngineState = EngineIdle | EngineComputing | EngineMove;
export const defaultEngine = (): EngineState => engineIdle();

export type MoveHist = {
  readonly _tag: "hist";
  move: Move;
  legals: Move[];
  fen: string;
};
export const moveHist = (
  move: Move,
  legals: Move[],
  fen: string
): MoveHist => ({
  _tag: "hist",
  move,
  legals,
  fen,
});

type GameConfig = {
  black: number;
  white: number;
  engineColor: Color;
  position: Nullable<string>;
};

export const gameConfig = (
  white: number,
  black: number,
  engineColor: Color,
  position = null as Nullable<string>
): GameConfig => ({ black, white, engineColor, position });

export type Eco = {
  name: string;
  code: string;
  fen: string;
  moves: Move[];
  pgn: string;
};

export type SavedGame = {
  hist: MoveHist[];
  config: GameConfig;
  outcome: Nullable<string>;
  timestamp: number;
};

export const savedGame = (
  hist: MoveHist[],
  config: GameConfig,
  outcome: Nullable<string>,
  timestamp: number
): SavedGame => ({
  hist,
  config,
  outcome,
  timestamp,
});
