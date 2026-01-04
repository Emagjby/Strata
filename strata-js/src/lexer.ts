import { ParseError } from "./parse_error.js";

export type Span = {
    offset: number;
    line: number;
    column: number;
};

export type Token = {
    kind: TokenKind;
    span: Span;
};

export type TokenKind =
    | { type: "Null" }
    | { type: "True" }
    | { type: "False" }
    | { type: "Int"; value: bigint }
    | { type: "String"; value: string }
    | { type: "Bytes"; value: Uint8Array }
    | { type: "Ident"; value: string }
    | { type: "LBrace" }
    | { type: "RBrace" }
    | { type: "LBracket" }
    | { type: "RBracket" }
    | { type: "Colon" }
    | { type: "Comma" }
    | { type: "EOF" };

const I64_MIN = -(1n << 63n);
const I64_MAX = (1n << 63n) - 1n;

export class Lexer {
    private offset = 0;
    private line = 1;
    private column = 1;

    constructor(private readonly input: string) { }

    private span(): Span {
        return { offset: this.offset, line: this.line, column: this.column };
    }

    private err(kind: any): ParseError {
        const span = this.span();
        return new ParseError(kind, span.offset, span.line, span.column);
    }

    private peek(): string | null {
        return this.offset < this.input.length ? this.input[this.offset]! : null;
    }

    private bump(): string | null {
        const char = this.peek();
        if (char === null) return null;

        this.offset++;

        if (char === "\n") {
            this.line++;
            this.column = 1;
        } else {
            this.column++;
        }

        return char;
    }

    private skipIgnored(): void {
        while (true) {
            // whitespace
            while (true) {
                const char = this.peek();
                if (char === " " || char === "\t" || char === "\n" || char === "\r") {
                    this.bump();
                } else {
                    break;
                }
            }

            // comment //
            if (this.peek() === "/" && this.input[this.offset + 1] === "/") {
                this.bump();
                this.bump();
                while (true) {
                    const char = this.peek();
                    if (char === null) {
                        break;
                    }
                    this.bump();
                    if (char === "\n") {
                        break;
                    }
                }
                continue;
            }

            // comment #
            if (this.peek() === "#") {
                this.bump();
                while (true) {
                    const char = this.peek();
                    if (char === null) {
                        break;
                    }
                    this.bump();
                    if (char === "\n") {
                        break;
                    }
                }
                continue;
            }

            break;
        }
    }

    private isIdentStart(char: string): boolean {
        return (
            (char >= "a" && char <= "z") ||
            (char >= "A" && char <= "Z") ||
            char === "_"
        );
    }

    private isIdentContinue(char: string): boolean {
        return this.isIdentStart(char) || (char >= "0" && char <= "9");
    }

    private lexIdentifier(): TokenKind {
        const start = this.offset;
        this.bump();

        while (true) {
            const char = this.peek();
            if (char !== null && this.isIdentContinue(char)) {
                this.bump();
            } else {
                break;
            }
        }

        const ident = this.input.slice(start, this.offset);

        if (ident === "null") return { type: "Null" };
        if (ident === "true") return { type: "True" };
        if (ident === "false") return { type: "False" };
        return { type: "Ident", value: ident };
    }

    private lexInt(): TokenKind {
        const start = this.offset;

        if (this.peek() === "-") this.bump();

        let sawDigit = false;
        while (true) {
            const char = this.peek();
            if (char !== null && char >= "0" && char <= "9") {
                sawDigit = true;
                this.bump();
            } else {
                break;
            }
        }

        if (!sawDigit) {
            throw this.err({ type: "IntegerOutOfRange" });
        }

        const text = this.input.slice(start, this.offset);

        let n: bigint;

        try {
            n = BigInt(text);
        } catch {
            throw this.err({ type: "IntegerOutOfRange" });
        }

        if (n < I64_MIN || n > I64_MAX) {
            throw this.err({ type: "IntegerOutOfRange" });
        }

        return { type: "Int", value: n };
    }

    private hexDigit(char: string): number | null {
        if (char >= "0" && char <= "9")
            return char.charCodeAt(0) - "0".charCodeAt(0);
        if (char >= "a" && char <= "f")
            return char.charCodeAt(0) - "a".charCodeAt(0) + 10;
        if (char >= "A" && char <= "F")
            return char.charCodeAt(0) - "A".charCodeAt(0) + 10;
        return null;
    }

    private lexBytes(): TokenKind {
        // must start with 0x
        if (!(this.peek() === "0" && this.input[this.offset + 1] === "x")) {
            throw this.err({ type: "MalformedBytesLiteral" });
        }

        this.bump(); // 0
        this.bump(); // x

        const hexStart = this.offset;

        while (true) {
            const char = this.peek();
            if (char !== null && this.hexDigit(char) !== null) {
                this.bump();
            } else {
                break;
            }
        }

        const hex = this.input.slice(hexStart, this.offset);

        if (hex.length === 0 || hex.length % 2 !== 0) {
            throw this.err({ type: "MalformedBytesLiteral" });
        }

        const out = new Uint8Array(hex.length / 2);
        for (let i = 0; i < hex.length; i += 2) {
            const hi = this.hexDigit(hex[i]!);
            const lo = this.hexDigit(hex[i + 1]!);
            if (hi === null || lo === null) {
                throw this.err({ type: "MalformedBytesLiteral" });
            }
            out[i / 2] = (hi << 4) | lo;
        }

        return { type: "Bytes", value: out };
    }

    private lexString(): TokenKind {
        this.bump(); // opening "

        let out = "";

        while (true) {
            const char = this.peek();
            if (char === null) {
                throw this.err({ type: "MalformedBytesLiteral" });
            }

            if (char === '"') {
                this.bump();
                return { type: "String", value: out };
            }

            if (char === "\\") {
                this.bump();
                const escape = this.bump();
                if (escape === null) {
                    throw this.err({ type: "MalformedStringLiteral" });
                }

                switch (escape) {
                    case '"':
                        out += '"';
                        break;
                    case "\\":
                        out += "\\";
                        break;
                    case "n":
                        out += "\n";
                        break;
                    case "r":
                        out += "\r";
                        break;
                    case "t":
                        out += "\t";
                        break;
                    case "u": {
                        // \uXXXX
                        let code = 0;
                        for (let i = 0; i < 4; i++) {
                            const hexChar = this.bump();
                            if (hexChar === null) {
                                throw this.err({ type: "MalformedStringLiteral" });
                            }
                            const digit = this.hexDigit(hexChar);
                            if (digit === null) {
                                throw this.err({ type: "MalformedStringLiteral" });
                            }
                            code = (code << 4) | digit;
                        }
                        out += String.fromCharCode(code);
                        break;
                    }
                    default:
                        throw this.err({ type: "MalformedStringLiteral" });
                }

                continue;
            }

            // Rust lexer rejects non-ASCII bytes in strings
            if (char.charCodeAt(0) >= 0x80) {
                throw this.err({ type: "MalformedStringLiteral" });
            }

            out += char;
            this.bump();
        }
    }

    nextToken(): Token {
        this.skipIgnored();
        const start = this.span();

        const char = this.peek();
        if (char === null) {
            return { kind: { type: "EOF" }, span: start };
        }

        const kind: TokenKind = (() => {
            switch (char) {
                case "{":
                    this.bump();
                    return { type: "LBrace" };
                case "}":
                    this.bump();
                    return { type: "RBrace" };
                case "[":
                    this.bump();
                    return { type: "LBracket" };
                case "]":
                    this.bump();
                    return { type: "RBracket" };
                case ":":
                    this.bump();
                    return { type: "Colon" };
                case ",":
                    this.bump();
                    return { type: "Comma" };
                case '"':
                    return this.lexString();
                case "0":
                    if (this.input[this.offset + 1] === "x") {
                        return this.lexBytes();
                    }
                    return this.lexInt();
                case "-":
                    return this.lexInt();
                default:
                    if (this.isIdentStart(char)) return this.lexIdentifier();
                    if (char >= "1" && char <= "9") return this.lexInt();
                    throw this.err({
                        type: "UnexpectedToken",
                        expected: "valid token",
                        found: "invalid character",
                    });
            }
        })();

        return { kind, span: start };
    }
}
