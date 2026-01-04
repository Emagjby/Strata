export type Value =
    | NullValue
    | BoolValue
    | IntValue
    | StringValue
    | BytesValue
    | ListValue
    | MapValue;

export interface NullValue {
    readonly kind: "null";
}

export interface BoolValue {
    readonly kind: "bool";
    readonly value: boolean;
}

export interface IntValue {
    readonly kind: "int";
    readonly value: bigint;
}

export interface StringValue {
    readonly kind: "string";
    readonly value: string;
}

export interface BytesValue {
    readonly kind: "bytes";
    readonly value: Uint8Array;
}

export interface ListValue {
    readonly kind: "list";
    readonly value: readonly Value[];
}

export interface MapValue {
    readonly kind: "map";
    readonly value: ReadonlyMap<string, Value>;
}
