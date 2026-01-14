import {
    Value as ValueT,
    NullValue,
    BoolValue,
    IntValue,
    StringValue,
    BytesValue,
    ListValue,
    MapValue,
} from "./value.js";

function assertByte(n: number): void {
    if (!Number.isInteger(n) || n < 0 || n > 255) {
        throw new TypeError("Byte values must be integers in range 0..255");
    }
}

function bytesFromIterable(it: Iterable<number>): Uint8Array {
    const out: number[] = [];
    for (const n of it) {
        assertByte(n);
        out.push(n);
    }
    return Uint8Array.from(out);
}

function bytesFromHexStrict(hex: string): Uint8Array {
    if (typeof hex !== "string") {
        throw new TypeError("Hex string must be a string");
    }
    if (hex.length % 2 !== 0) {
        throw new TypeError("Hex string must have even length");
    }
    if (!/^[0-9a-fA-F]*$/.test(hex)) {
        throw new TypeError("Hex string contains invalid characters");
    }

    const out = new Uint8Array(hex.length / 2);

    for (let i = 0; i < hex.length; i += 2) {
        out[i / 2] = Number.parseInt(hex.slice(i, i + 2), 16);
    }
    return out;
}

export const Value = {
    null(): NullValue {
        return { kind: "null" };
    },

    bool(value: boolean): BoolValue {
        return { kind: "bool", value };
    },

    int(value: bigint): IntValue {
        if (typeof value !== "bigint") {
            throw new TypeError("Strata Int must be a bigint");
        }
        return { kind: "int", value };
    },

    string(value: string): StringValue {
        return { kind: "string", value };
    },

    bytes(value: Uint8Array): BytesValue {
        if (!(value instanceof Uint8Array)) {
            throw new TypeError("Strata Bytes must be a Uint8Array");
        }
        return { kind: "bytes", value };
    },

    bytesFrom(
        value: ArrayBuffer | Uint8Array | readonly number[] | Iterable<number>,
    ): BytesValue {
        if (value instanceof Uint8Array) {
            return Value.bytes(value);
        }
        if (value instanceof ArrayBuffer) {
            return Value.bytes(new Uint8Array(value));
        }
        if (Array.isArray(value)) {
            for (const n of value) assertByte(n);
            return Value.bytes(Uint8Array.from(value));
        }
        return Value.bytes(bytesFromIterable(value));
    },

    bytesHex(hex: string): BytesValue {
        return Value.bytes(bytesFromHexStrict(hex));
    },

    list(value: readonly ValueT[]): ListValue {
        return { kind: "list", value };
    },

    listOf(...values: ValueT[]): ListValue {
        return { kind: "list", value: values };
    },

    map(entries: Iterable<[string, ValueT]>): MapValue {
        return { kind: "map", value: new Map(entries) };
    },

    mapObj(obj: Record<string, ValueT>): MapValue {
        return {
            kind: "map",
            value: new Map(Object.entries(obj)),
        };
    },

    mapOf(...entries: [string, ValueT][]): MapValue {
        return { kind: "map", value: new Map(entries) };
    },
} as const;

export const V = Value;
