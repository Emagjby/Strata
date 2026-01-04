import { Value } from "./value.js";

export function inspectValue(value: Value): any {
    switch (value.kind) {
        case "null":
            return null;

        case "int":
            return value.value.toString();

        case "bool":
        case "string":
            return value.value;

        case "bytes":
            return Array.from(value.value);

        case "list":
            return value.value.map(inspectValue);

        case "map": {
            const obj: Record<string, any> = {};
            for (const [k, v] of value.value.entries()) {
                obj[k] = inspectValue(v);
            }
            return obj;
        }
    }
}
