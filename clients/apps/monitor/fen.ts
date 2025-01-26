/**
 * 
 * From https://ia802908.us.archive.org/26/items/pgn-standard-1994-03-12/PGN_standard_1994-03-12.txt
 * 
 * 16.1.3.1: Piece placement data
 * 
 * The first field represents the placement of the pieces on the board.  The board
 * contents are specified starting with the eighth rank and ending with the first
 * rank.  For each rank, the squares are specified from file a to file h.  White
 * pieces are identified by uppercase SAN piece letters ("PNBRQK") and black
 * pieces are identified by lowercase SAN piece letters ("pnbrqk").  Empty squares
 * are represented by the digits one through eight; the digit used represents the
 * count of contiguous empty squares along a rank.  A solidus character "/" is
 * used to separate data of adjacent ranks.

 */

import {
  Role,
  Nullable,
  Color,
  Square,
  squareFiles,
  squareRanks,
  makeSquare,
} from "../lib/ucui/types";

const rolesIds: [string, Role][] = [
  ["P", "Pawn"],
  ["B", "Bishop"],
  ["N", "Knight"],
  ["R", "Rook"],
  ["Q", "Queen"],
  ["K", "King"],
];

export type SquareOccup = {
  role: Role;
  color: Color;
};

// type Occup = Nullable<[Role, Color]>;

const upperInitials = rolesIds.map((r) => r[0]);

const occupFor = (c: string): [number, Nullable<SquareOccup>] => {
  const n = parseInt(c, 10);
  if (!Number.isNaN(n)) {
    return [n, null];
  }
  const roleTuple = rolesIds.find((r) => r[0] === c.toUpperCase())!;
  const color = upperInitials.indexOf(c) >= 0 ? "white" : "black";

  return [1, { role: roleTuple[1], color }];
};

export type OccupProc<R> = (square: Square, occup: Nullable<SquareOccup>) => R;

export const fenToRanks = <R>(fen: string, proc: OccupProc<R>): R[][] => {
  try {
    const pieces = fen.split(" ")[0];
    const rankStrings = pieces.split("/");
    return squareRanks.map((r, ri) => {
      const rankString = rankStrings[ri]!;
      const occups: Nullable<SquareOccup>[] = squareFiles.map(() => null);
      let current = 0;
      rankString.split("").forEach((c) => {
        const [n, occup] = occupFor(c);
        for (let i = 0; i < n; i++) {
          occups[current + i] = occup;
        }
        current += n;
      });
      return squareFiles.map<R>((f, fi) => proc(makeSquare(f, r), occups[fi]));
    });
  } catch (error) {
    console.error("Failed to procees FEN", error);
    return [[]];
  }
};

// const roleLetter = (role: Role) => {
//   switch (role) {
//     case "Pawn":
//       return "♙";
//     case "Rook":
//       return "♖";
//     case "Knight":
//       return "♘";
//     case "Bishop":
//       return "♗";
//     case "Queen":
//       return "♕";
//     case "King":
//       return "♔";
//   }
// };

// const rs = (r: Occup[]) =>
//   r.map((o) =>
//     o === null
//       ? "."
//       : o[1] === "white"
//       ? roleLetter(o[0])
//       : roleLetter(o[0]).toLocaleLowerCase()
//   );

// const fen = "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2";
// const board = fenToBoard(fen);
// board.forEach((r) => console.log(rs(r).join(" ")));

// const text = (t: string) => document.createTextNode(t);

// const div = (className: string, ...t: (Element | Text)[]) => {
//   const e = document.createElement("div");
//   e.setAttribute("class", className);
//   e.append(...t);
//   return e;
// };

// const fenTo = (fen: string) =>
//   fenToBoard(fen).map((r) =>
//     r.map((o) =>
//       o === null
//         ? div("empty", text("."))
//         : o[1] === "white"
//         ? div("piece", text(roleLetter(o[0])))
//         : div("piece", text(roleLetter(o[0]).toLocaleLowerCase()))
//     )
//   );

// const appendFen = (fen: string, root: HTMLElement) =>
//   fenTo(fen).map((r) => root.append(div("rank", ...r)));

// const root = document.getElementById("game");

// appendFen(
//   "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2",
//   root!
// );
