import { Value } from "./value.js";

export enum FormatOptions {
    PRETTY = "PRETTY",
    AST = "AST",
}

export function fmt(opts: FormatOptions, value: Value): string {
    switch (opts) {
        case FormatOptions.PRETTY: {
            const f = new PrettyFmt();
            f.value(value);
            return f.out;
        }
        case FormatOptions.AST:
            return prettyDebug(value);
    }
}

class PrettyFmt {
    out = "";
    depth = 0;
    indent = 2;

    private pad(): string {
        return " ".repeat(this.depth * this.indent);
    }

    value(v: Value): void {
        switch (v.kind) {
            case "null":
                this.out += "null";
                break;
            case "bool":
                this.out += String(v.value);
                break;
            case "int":
                this.out += String(v.value);
                break;
            case "string":
                this.out += `"${v.value}"`;
                break;
            case "bytes":
                this.bytes(v.value);
                break;
            case "list":
                this.list(v.value as Value[]);
                break;
            case "map":
                this.map(v.value as Map<string, Value>);
                break;
        }
    }

    private bytes(bytes: Uint8Array): void {
        this.out += "[";
        for (const b of bytes) {
            this.out += b.toString(16).padStart(2, "0");
        }
        this.out += "]";
    }

    private list(list: Value[]): void {
        this.out += "[";
        for (let i = 0; i < list.length; i++) {
            this.value(list[i]!);
            if (i + 1 !== list.length) this.out += ", ";
        }
        this.out += "]";
    }

    private map(map: Map<string, Value>): void {
        this.out += "{\n";
        this.depth += 1;

        const entries = [...map.entries()].sort(([a], [b]) => a.localeCompare(b));

        for (const [k, v] of entries) {
            this.out += this.pad();
            this.out += k;
            this.out += ": ";
            this.value(v);
            this.out += "\n";
        }

        this.depth -= 1;
        this.out += this.pad();
        this.out += "}";
    }
}

function prettyDebug(value: unknown): string {
    return JSON.stringify(value, null, 2);
}
