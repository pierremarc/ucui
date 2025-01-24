import { emptyElement, events } from "./lib/dom";
import { DIV, replaceNodeContent } from "./lib/html";
import { sendMove } from "./play";
import { formatMove } from "./san";
import {
  assign,
  Color,
  dispatch,
  get,
  getInputRole,
  getMoveRole,
  getTurn,
  inputMove,
  inputRole,
  Move,
  moveHist,
  Nullable,
  Role,
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

const renderMoves = (selected: Nullable<Role>, moveList: Move[]) =>
  moveList
    .filter((m) => getMoveRole(m) === selected)
    .map((move) =>
      events(DIV("move", formatMove(move, moveList)), (add) =>
        add("click", () => {
          assign("input", inputMove(move));
          dispatch("moveList", (list) =>
            list.concat(moveHist(move, get("position").legalMoves))
          );
          sendMove(move);
        })
      )
    );

export const mountInput = (root: HTMLElement) => {
  const pieces = DIV("pieces");
  const moves = DIV("moves");
  const inputElement = DIV("input", moves, pieces);
  root.append(inputElement);

  const update = () => {
    if (getTurn() !== get("gameConfig").engineColor) {
      const replacePieces = replaceNodeContent(pieces);
      const replaceMoves = replaceNodeContent(moves);
      const pos = get("position");
      const input = get("input");
      const selectedRole = getInputRole(input);
      replacePieces(...renderPieces(selectedRole, pos.legalMoves));
      if (input._tag === "role") {
        replaceMoves(...renderMoves(input.role, pos.legalMoves));
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
