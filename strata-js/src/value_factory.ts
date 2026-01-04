import {
    Value,
    NullValue,
    BoolValue,
    IntValue,
    StringValue,
    BytesValue,
    ListValue,
    MapValue,
} from "./value.js";

export const V = {
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

    list(value: readonly Value[]): ListValue {
        return { kind: "list", value };
    },

    map(entries: Iterable<[string, Value]>): MapValue {
        return { kind: "map", value: new Map(entries) };
    },
} as const;
