import { Lexer, Token, TokenKind } from "./lexer.js";
import { ParseError } from "./parse_error.js";
import { Value } from "./value.js";
import { V } from "./value_factory.js";

type TokenType = TokenKind["type"];

export class Parser {
    private lexer: Lexer;
    private lookahead: Token;

    constructor(input: string) {
        this.lexer = new Lexer(input);
        this.lookahead = this.lexer.nextToken();
    }

    private get type(): TokenType {
        return this.lookahead.kind.type;
    }

    private advance(): void {
        this.lookahead = this.lexer.nextToken();
    }

    private unexpected(expected: string, found: string): never {
        const s = this.lookahead.span;
        throw new ParseError(
            {
                type: "UnexpectedToken",
                expected,
                found,
            },
            s.offset,
            s.line,
            s.column,
        );
    }

    private expect(type: TokenType): void {
        if (this.type === type) {
            this.advance();
        } else {
            this.unexpected(type, this.lookahead.kind.type);
        }
    }

    private parseList(): Value {
        this.expect("LBracket");

        const items: Value[] = [];

        if (this.type === "RBracket") {
            this.advance();
            return V.list(items);
        }

        while (true) {
            items.push(this.parseValue());

            if ((this.type as TokenType) === "Comma") {
                this.advance();

                if ((this.type as TokenType) === "RBracket") {
                    break;
                }
                continue;
            }

            if ((this.type as TokenType) === "RBracket") {
                break;
            }

            this.unexpected("',' or ']'", this.type);
        }

        this.expect("RBracket");
        return V.list(items);
    }

    private parseMap(): Value {
        this.expect("LBrace");

        const map = new Map<string, Value>();

        if (this.type === "RBrace") {
            this.advance();
            return V.map(map);
        }

        while (true) {
            if (this.type !== "Ident") {
                this.unexpected("identifier", this.type);
            }

            const tok = this.lookahead;

            if (tok.kind.type !== "Ident") {
                this.unexpected("identifier", tok.kind.type);
            }

            const key = tok.kind.value;
            this.advance();

            // shorthand: key { ... }
            let value: Value;

            if (this.lookahead.kind.type === "LBrace") {
                value = this.parseMap();
            } else {
                this.expect("Colon");
                value = this.parseValue();
            }

            map.set(key, value);

            if ((this.type as TokenType) === "Comma") {
                this.advance();

                if ((this.type as TokenType) === "RBrace") {
                    break;
                }
                continue;
            }

            if ((this.type as TokenType) === "RBrace") {
                break;
            }

            // newline separator
            if (this.type === "Ident") {
                continue;
            }

            this.unexpected("',' or '}'", this.type);
        }

        this.expect("RBrace");
        return V.map(map);
    }

    private parseValue(): Value {
        const kind = this.lookahead.kind;

        switch (kind.type) {
            case "Null":
                this.advance();
                return V.null();

            case "True":
                this.advance();
                return V.bool(true);

            case "False":
                this.advance();
                return V.bool(false);

            case "Int": {
                const value = kind.value;
                this.advance();
                return V.int(value);
            }

            case "String": {
                const value = kind.value;
                this.advance();
                return V.string(value);
            }

            case "Bytes": {
                const value = kind.value;
                this.advance();
                return V.bytes(value);
            }

            case "LBracket":
                return this.parseList();

            case "LBrace":
                return this.parseMap();

            case "Ident": {
                const name = kind.value;
                this.advance();

                if (this.type === "LBrace") {
                    const inner = this.parseMap();
                    return V.map([[name, inner] as [string, Value]]);
                }

                this.unexpected("map or value", "identifier");
            }

            default:
                this.unexpected("value", kind.type);
        }
    }

    parseRoot(): Value {
        const value = this.parseValue();
        if (this.type !== "EOF") {
            this.unexpected("EOF", "extra input");
        }
        return value;
    }
}

export function parse(input: string): Value {
    const parser = new Parser(input);
    return parser.parseRoot();
}
