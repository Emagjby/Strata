export type DecodeErrorKind =
    | { type: "InvalidTag"; tag: number }
    | { type: "UnexpectedEOF" }
    | { type: "InvalidVarint" }
    | { type: "InvalidUTF8" }
    | { type: "TrailingBytes" };

export class DecodeError extends Error {
    readonly kind: DecodeErrorKind;
    readonly offset: number;

    constructor(kind: DecodeErrorKind, offset: number) {
        super(`DecodeError at ${offset}`);
        this.kind = kind;
        this.offset = offset;
    }
}
