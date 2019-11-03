export const floats_equal = (f1:number) => (f2:number) =>
    Math.abs(f1 - f2) < Number.EPSILON;

export const find_map = <K,V>(pred: (key:K) => boolean) => (m:Map<K,V>):V | undefined => {
    for (var [k, v] of m) {
        if(pred(k)) {
            return v;
        }
    }

    return undefined;
}

export const find_object = <V>(pred: (key:string) => boolean) => (obj:{[key:string]: V}):V | undefined => {
    const key = Object.keys(obj).find(key => pred(key));

    return key ? obj[key] : undefined;
}

export const append_strings = (preds:Array<() => [string, boolean]>) => (initial:string) =>
  preds.reduce((acc, x) => {
    const [value, flag] = x();
    return flag ? `${acc} ${value}` : acc;
  }, initial)