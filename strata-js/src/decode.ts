import { Value } from "./value.js";
import { V } from "./value_factory.js";
import { decodeULEB128, decodeSLEB128 } from "./varint.js";
import { DecodeError } from "./decode_error.js";

const td = new TextDecoder("utf-8", { fatal: true });

const TAG_NULL = 0x00;
const TAG_FALSE = 0x01;
const TAG_TRUE = 0x02;

const TAG_INT = 0x10;
const TAG_STRING = 0x20;
const TAG_BYTES = 0x21;

const TAG_LIST = 0x30;
const TAG_MAP = 0x40;

class Decoder {
    private offset = 0;

    constructor(private readonly input: Uint8Array) { }

    private remaining(): number {
        return this.input.length - this.offset;
    }

    private readByte(): number {
        if (this.offset >= this.input.length) {
            throw new DecodeError({ type: "UnexpectedEOF" }, this.offset);
        }

        return this.input[this.offset++]!;
    }

    private readSlice(len: number): Uint8Array {
        if (this.remaining() < len) {
            throw new DecodeError({ type: "UnexpectedEOF" }, this.offset);
        }

        const slice = this.input.subarray(this.offset, this.offset + len);
        this.offset += len;
        return slice;
    }

    decodeValue(): Value {
        const tagOffset = this.offset;
        const tag = this.readByte();

        switch (tag) {
            case TAG_NULL:
                return V.null();

            case TAG_FALSE:
                return V.bool(false);

            case TAG_TRUE:
                return V.bool(true);

            case TAG_INT: {
                try {
                    const { value, nextOffset } = decodeSLEB128(this.input, this.offset);
                    this.offset = nextOffset;
                    return V.int(value);
                } catch {
                    throw new DecodeError({ type: "InvalidVarint" }, tagOffset);
                }
            }

            case TAG_STRING: {
                let len: bigint;
                try {
                    const result = decodeULEB128(this.input, this.offset);
                    len = result.value;
                    this.offset = result.nextOffset;
                } catch {
                    throw new DecodeError({ type: "InvalidVarint" }, tagOffset);
                }

                const start = this.offset;
                const bytes = this.readSlice(Number(len));

                try {
                    const str = td.decode(bytes);
                    return V.string(str);
                } catch {
                    throw new DecodeError({ type: "InvalidUTF8" }, start);
                }
            }

            case TAG_BYTES: {
                let len: bigint;
                try {
                    const result = decodeULEB128(this.input, this.offset);
                    len = result.value;
                    this.offset = result.nextOffset;
                } catch {
                    throw new DecodeError({ type: "InvalidVarint" }, tagOffset);
                }

                const bytes = this.readSlice(Number(len));
                return V.bytes(bytes);
            }

            case TAG_LIST: {
                let count: bigint;

                try {
                    const result = decodeULEB128(this.input, this.offset);
                    count = result.value;
                    this.offset = result.nextOffset;
                } catch {
                    throw new DecodeError({ type: "InvalidVarint" }, tagOffset);
                }

                const items: Value[] = [];
                for (let i = 0n; i < count; i++) {
                    items.push(this.decodeValue());
                }

                return V.list(items);
            }

            case TAG_MAP: {
                let count: bigint;
                try {
                    const result = decodeULEB128(this.input, this.offset);
                    count = result.value;
                    this.offset = result.nextOffset;
                } catch {
                    throw new DecodeError({ type: "InvalidVarint" }, tagOffset);
                }

                const map = new Map<string, Value>();

                for (let i = 0n; i < count; i++) {
                    const keyValue = this.decodeValue();
                    if (keyValue.kind !== "string") {
                        throw new DecodeError({ type: "InvalidTag", tag }, this.offset);
                    }

                    const value = this.decodeValue();
                    map.set(keyValue.value, value);
                }

                return V.map(map);
            }

            default:
                throw new DecodeError({ type: "InvalidTag", tag }, tagOffset);
        }
    }
}

export function decodeValue(input: Uint8Array): Value {
    const decoder = new Decoder(input);
    const value = decoder.decodeValue();

    if (decoder["remaining"]() !== 0) {
        throw new DecodeError(
            { type: "TrailingBytes" },
            input.length - decoder["remaining"](),
        );
    }

    return value;
}
