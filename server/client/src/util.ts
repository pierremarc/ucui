import { Role } from "./store";

export const ROLE_LIST: Role[] = [
  "Pawn",
  "Bishop",
  "Knight",
  "Rook",
  "Queen",
  "King",
];

export const BLACK_PAWN = "♟";
export const BLACK_ROOK = "♜";
export const BLACK_KNIGHT = "♞";
export const BLACK_BISHOP = "♝";
export const BLACK_QUEEN = "♛";
export const BLACK_KING = "♚";

export const WHITE_PAWN = "♙";
export const WHITE_ROOK = "♖";
export const WHITE_KNIGHT = "♘";
export const WHITE_BISHOP = "♗";
export const WHITE_QUEEN = "♕";
export const WHITE_KING = "♔";

export type EncodableLiteral = string | number | boolean;
export type Encodable = EncodableLiteral | EncodableLiteral[] | null;
export type UrlQuery = Record<string, Encodable>;

const encodeComponent = (key: string, value: Encodable): string => {
  if (Array.isArray(value)) {
    return value.map((v) => `${key}=${encodeURIComponent(v)}`).join("&");
  }
  return value == null ? `${key}=` : `${key}=${encodeURIComponent(value)}`;
};

export const withQueryString = (url: string, attrs: UrlQuery) => {
  const qs = Object.keys(attrs)
    .map((k) => encodeComponent(k, attrs[k]))
    .join("&");
  return `${url}?${qs}`;
};
