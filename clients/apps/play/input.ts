import { events, emptyElement } from "../lib/dom";
import { DIV, replaceNodeContent } from "../lib/html";
import {
  Role,
  Color,
  Move,
  Nullable,
  inputRole,
  inputMove,
  moveHist,
  getMoveRole,
  Square,
  squareRanks,
  squareFiles,
  makeSquare,
  getInputRole,
} from "../lib/ucui/types";
import { sendMove } from "./play";
import { formatMove } from "./san";
import {
  assign,
  dispatch,
  getPlayerColor,
  getTurn,
  get,
  subscribe,
} from "./store";

import {
  BLACK_PAWN,
  WHITE_PAWN,
  BLACK_ROOK,
  WHITE_ROOK,
  BLACK_KNIGHT,
  WHITE_KNIGHT,
  BLACK_BISHOP,
  WHITE_BISHOP,
  BLACK_QUEEN,
  WHITE_QUEEN,
  BLACK_KING,
  WHITE_KING,
  ROLE_LIST,
  show,
} from "./util";

const symbol = (role: Role, color: Color) => {
  switch (role) {
    case "Pawn":
      return color === "black" ? BLACK_PAWN : WHITE_PAWN;
    case "Rook":
      return color === "black" ? BLACK_ROOK : WHITE_ROOK;
    case "Knight":
      return color === "black" ? BLACK_KNIGHT : WHITE_KNIGHT;
    case "Bishop":
      return color === "black" ? BLACK_BISHOP : WHITE_BISHOP;
    case "Queen":
      return color === "black" ? BLACK_QUEEN : WHITE_QUEEN;
    case "King":
      return color === "black" ? BLACK_KING : WHITE_KING;
  }
};

const hasMoves = (role: Role, moveList: Move[]) =>
  moveList.some((m) => {
    switch (m._tag) {
      case "Normal":
        return m.role === role;
      case "Castle":
        return role === "King";
      case "EnPassant":
        return role === "Pawn";
    }
  });

const selClass = (s: boolean) => (s ? "selected" : "");

// const hasMovesClass = (s: boolean) => (s ? "has-moves" : "has-no-moves");

const renderPieces = (selected: Nullable<Role>, moveList: Move[]) =>
  ROLE_LIST.map((role) =>
    events(
      DIV(
        `piece ${role}  ${selClass(selected === role)}`,
        symbol(role, hasMoves(role, moveList) ? "black" : "white")
      ),
      (add) => add("click", () => assign("input", inputRole(role)))
    )
  );

const playMove = (move: Move) => {
  assign("input", inputMove(move));
  dispatch("moveList", (list) =>
    list.concat(moveHist(move, get("position").legalMoves))
  );
  sendMove(move);
};

const renderMoves = (selected: Nullable<Role>, moveList: Move[]) =>
  moveList
    .filter((m) => getMoveRole(m) === selected)
    .map((move) =>
      events(DIV("move", formatMove(move, moveList)), (add) =>
        add("click", () => playMove(move))
      )
    );

const findAt = (candidates: Move[]) => (s: Square) =>
  candidates.filter((move) => {
    switch (move._tag) {
      case "Castle": {
        switch (s) {
          case "G1":
            return move.king === "E1" && move.rook == "H1";
          case "C1":
            return move.king === "E1" && move.rook == "A1";
          case "G8":
            return move.king === "E8" && move.rook == "H8";
          case "C8":
            return move.king === "E8" && move.rook == "A8";
        }
        return false;
      }

      case "Normal":
      case "EnPassant":
        return move.to === s;
    }
  });
const getRole = (move: Move): Role => {
  switch (move._tag) {
    case "Castle":
      return "King";
    case "Normal":
      return move.role;
    case "EnPassant":
      return "Pawn";
  }
};

const renderMoves2 = (selected: Nullable<Role>, moveList: Move[]) => {
  const candidates = moveList.filter((m) => getMoveRole(m) === selected);
  const find = findAt(candidates);
  const orderedRanks =
    getPlayerColor() === "black" ? squareRanks : squareRanks.slice(0).reverse();
  const orderedFiles =
    getPlayerColor() === "white" ? squareFiles : squareFiles.slice(0).reverse();
  const filesRank = DIV(
    "rank",
    ...[DIV("ord")]
      .concat(orderedFiles.map((f) => DIV("ord", f.toLowerCase())))
      .concat(DIV("ord"))
  );

  const selectElement = DIV("select hidden");

  const replaceSelect = replaceNodeContent(selectElement);
  const renderSelect = (moves: Move[]) => {
    replaceSelect(
      ...moves.map((move) =>
        events(DIV("move", formatMove(move, moveList)), (add) =>
          add("click", () => playMove(move))
        )
      )
    );
    show(selectElement);
  };
  const squares = orderedRanks.map((rank) =>
    DIV(
      `rank ${rank}`,
      DIV("ord", rank),
      ...orderedFiles.map((file) => {
        const square = makeSquare(file, rank);
        const tos = find(square);
        if (tos.length == 0) {
          return DIV(`square ${square}`);
        } else {
          return events(
            DIV(`square ${square} target`, DIV("label", square.toLowerCase())),
            (add) =>
              add("click", () => {
                // if (tos.length > 1) {
                renderSelect(tos);
                // } else {
                // playMove(tos[0]);
                // }
              })
          );
        }
      }),
      DIV("ord")
    )
  );

  return [selectElement, ...squares.concat(filesRank)];
};

export const mountInput = (root: HTMLElement) => {
  const pieces = DIV("pieces");
  const moves = DIV("moves");
  const inputElement = DIV("input", moves, pieces);
  root.append(inputElement);

  const update = () => {
    if (getTurn() === getPlayerColor()) {
      const replacePieces = replaceNodeContent(pieces);
      const replaceMoves = replaceNodeContent(moves);
      const pos = get("position");
      const input = get("input");
      const selectedRole = getInputRole(input);
      replacePieces(...renderPieces(selectedRole, pos.legalMoves));
      if (input._tag === "role") {
        // replaceMoves(...renderMoves(input.role, pos.legalMoves));
        replaceMoves(...renderMoves2(input.role, pos.legalMoves));
      } else {
        emptyElement(moves);
      }
    } else {
      emptyElement(moves);
      emptyElement(pieces);
    }
  };
  subscribe("position", "input", "moveList")(update);

  update();
};
