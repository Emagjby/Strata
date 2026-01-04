export type ParseErrorKind =
    | {
        type: "UnexpectedToken";
        expected: string;
        found: string;
    }
    | { type: "MalformedBytesLiteral" }
    | { type: "IntegerOutOfRange" };

export class ParseError extends Error {
    readonly kind: ParseErrorKind;
    readonly offset: number;
    readonly line: number;
    readonly column: number;

    constructor(
        kind: ParseErrorKind,
        offset: number,
        line: number,
        column: number,
    ) {
        super(`ParseError at ${line}:${column}`);
        this.kind = kind;
        this.offset = offset;
        this.line = line;
        this.column = column;
    }
}
