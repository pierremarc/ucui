import {
  Role,
  Square,
  Nullable,
  SquareRank,
  SquareFile,
  Move,
  getFile,
  getRank,
} from "../lib/ucui/types";
import {
  WHITE_PAWN,
  WHITE_ROOK,
  WHITE_KNIGHT,
  WHITE_BISHOP,
  WHITE_QUEEN,
  WHITE_KING,
} from "./util";

export type CastlingSide = "KingSide" | "QueenSide";

type SanNormal = {
  readonly _tag: "Normal";
  role: Role;
  capture: boolean;
  to: Square;
  rank: Nullable<SquareRank>;
  file: Nullable<SquareFile>;
  promotion: Nullable<Role>;
};

const sanNormal = (
  role: Role,
  file = null as Nullable<SquareFile>,
  rank = null as Nullable<SquareRank>,
  capture: boolean,
  to: Square,
  promotion = null as Nullable<Role>
): SanNormal => ({ _tag: "Normal", role, capture, to, rank, file, promotion });

type SanCastle = {
  readonly _tag: "Castle";
  side: CastlingSide;
};

const sanCastle = (side: CastlingSide): SanCastle => ({
  _tag: "Castle",
  side,
});

type SanNull = {
  readonly _tag: "null";
};

export type San = SanNormal | SanCastle | SanNull;

const sanCandidates = (legalMoves: Move[], role: Role, to: Square): Move[] =>
  legalMoves.filter((move) => {
    switch (move._tag) {
      case "Castle":
        return false;
      case "EnPassant":
        return role == "Pawn" && move.to === to;
      default:
        return to === move.to && role === move.role;
    }
  });

const disambiguate = (m: Move, moves: Move[]): San => {
  switch (m._tag) {
    case "Normal": {
      if (m.role === "Pawn") {
        return sanNormal(
          "Pawn",
          m.capture !== null ? getFile(m.from) : null,
          null,
          m.capture !== null,
          m.to,
          m.promotion
        );
      } else {
        let ambiguous = false;
        let ambiguous_file = false;
        let ambiguous_rank = false;
        for (const candidate of moves) {
          if (candidate._tag === "Normal") {
            if (
              m.from != candidate.from &&
              m.role == candidate.role &&
              m.to == candidate.to &&
              m.promotion == candidate.promotion
            ) {
              ambiguous = true;
              if (getRank(m.from) == getRank(candidate.from)) {
                ambiguous_rank = true;
              }
              if (getFile(m.from) == getFile(candidate.from)) {
                ambiguous_file = true;
              }
            }
          }
        }
        return sanNormal(
          m.role,
          ambiguous && (!ambiguous_file || ambiguous_rank)
            ? getFile(m.from)
            : null,
          ambiguous_file ? getRank(m.from) : null,
          m.capture !== null,
          m.to,
          m.promotion
        );
      }
    }
    case "EnPassant":
      return sanNormal("Pawn", getFile(m.from), null, true, m.to, null);
    case "Castle": {
      if (getFile(m.rook) < getFile(m.king)) {
        return sanCastle("QueenSide");
      } else {
        return sanCastle("KingSide");
      }
    }
  }
};

const roleLetter = (role: Role) => {
  switch (role) {
    case "Pawn":
      return "P";
    case "Rook":
      return "R";
    case "Knight":
      return "N";
    case "Bishop":
      return "B";
    case "Queen":
      return "Q";
    case "King":
      return "K";
  }
};

const roleSymbol = (role: Role) => {
  switch (role) {
    case "Pawn":
      return WHITE_PAWN;
    case "Rook":
      return WHITE_ROOK;
    case "Knight":
      return WHITE_KNIGHT;
    case "Bishop":
      return WHITE_BISHOP;
    case "Queen":
      return WHITE_QUEEN;
    case "King":
      return WHITE_KING;
  }
};

export const fromMove = (legalMoves: Move[], move: Move): San => {
  let legals: Move[] = [];
  if (move._tag === "Normal" && move.role !== "Pawn") {
    legals = sanCandidates(legalMoves, move.role, move.to);
  }
  return disambiguate(move, legals);
};

const toString = (san: San, symbol: boolean) => {
  const result: string[] = [];
  switch (san._tag) {
    case "Normal": {
      if (san.role !== "Pawn") {
        result.push(symbol ? roleSymbol(san.role) : roleLetter(san.role));
      }
      if (san.file !== null) {
        result.push(san.file.toLowerCase());
      }
      if (san.rank !== null) {
        result.push(san.rank);
      }
      if (san.capture) {
        result.push("x");
      }
      result.push(san.to.toLowerCase());
      if (san.promotion !== null) {
        result.push(
          "=",
          symbol ? roleSymbol(san.promotion) : roleLetter(san.promotion)
        );
      }
      break;
    }
    case "Castle": {
      if (san.side === "KingSide") {
        result.push("O", "-", "O");
      } else {
        result.push("O", "-", "O", "-", "O");
      }
      break;
    }
    case "null":
      result.push("--");
  }
  return result.join("");
};

export const formatMove = (move: Move, legals: Move[], symbol = true) =>
  toString(fromMove(legals, move), symbol);
