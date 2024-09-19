function convertToNumber(size: string): number {
    const units = {
        B: 1,
        KB: 1024,
        MB: 1024 ** 2,
        GB: 1024 ** 3,
        TB: 1024 ** 4,
        PB: 1024 ** 5,
    };

    const match = size.match(/^(\d+(?:\.\d+)?)\s*(B|KB|MB|GB|TB|PB)$/i);
    if (!match) {
        throw new Error("Invalid size format");
    }

    const value = parseFloat(match[1]);
    const unit = match[2].toUpperCase();

    //@ts-ignore
    return value * units[unit];
}

export function getDiskPourcentage(used: string, max: string): number {
    let maxNumber = convertToNumber(max);
    let usedNumber = convertToNumber(used);

    return (usedNumber / maxNumber) * 100;
}