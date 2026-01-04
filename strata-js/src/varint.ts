export function encodeULEB128(value: bigint): Uint8Array {
    if (value < 0n) {
        throw new RangeError(
            "ULEB128 encoding only supports non-negative integers",
        );
    }

    const out: number[] = [];

    let v = value;
    do {
        let byte = Number(v & 0x7fn);
        v >>= 7n;

        if (v !== 0n) {
            byte |= 0x80;
        }

        out.push(byte);
    } while (v !== 0n);

    return new Uint8Array(out);
}

export function decodeULEB128(
    bytes: Uint8Array,
    offset = 0,
): { value: bigint; nextOffset: number } {
    let result = 0n;
    let shift = 0n;
    let position = offset;

    while (true) {
        if (position >= bytes.length) {
            throw new RangeError("ULEB128 decoding exceeded input length");
        }

        const byte = bytes[position++];
        if (byte === undefined) {
            throw new RangeError("ULEB128 decoding exceeded input length");
        }

        const value = BigInt(byte & 0x7f);

        if (shift >= 64n) {
            throw new RangeError("ULEB128 value exceeds maximum safe integer size");
        }

        result |= value << shift;

        if ((byte & 0x80) === 0) {
            return { value: result, nextOffset: position };
        }

        shift += 7n;
    }
}

export function encodeSLEB128(value: bigint): Uint8Array {
    const out: number[] = [];
    let v = value;

    while (true) {
        let byte = Number(v & 0x7fn);
        const signBit = byte & 0x40;
        v >>= 7n;

        const done = (v === 0n && signBit === 0) || (v === -1n && signBit !== 0);

        if (!done) {
            byte |= 0x80;
        }

        out.push(byte);

        if (done) {
            break;
        }
    }

    return Uint8Array.from(out);
}

export function decodeSLEB128(
    bytes: Uint8Array,
    offset = 0,
): { value: bigint; nextOffset: number } {
    let result = 0n;
    let shift = 0n;
    let position = offset;
    let byte: number | undefined;

    while (true) {
        if (position >= bytes.length) {
            throw new RangeError("SLEB128 decoding exceeded input length");
        }

        byte = bytes[position++];
        if (byte === undefined) {
            throw new RangeError("SLEB128 decoding exceeded input length");
        }

        const value = BigInt(byte & 0x7f);

        result |= value << shift;
        shift += 7n;

        if ((byte & 0x80) === 0) {
            break;
        }

        if (shift >= 64n) {
            throw new RangeError("SLEB128 value exceeds maximum safe integer size");
        }
    }

    if (shift < 64n && (byte & 0x40) !== 0) {
        result |= -1n << shift;
    }

    return { value: result, nextOffset: position };
}
