const BASE62_ALPHABET = 'ZW4H1vR0KoLezB2YugXx7Vycj6NQsbl3ni8wm5GPrMFkUIpJTdAChtfSO9DqEa';

export const BigInttoBase62 = (num: bigint): string => {
  if (num === 0n) return '0';
  let result = '';
  const base = 62n;
  while (num > 0n) {
    const remainder = num % base;
    result = BASE62_ALPHABET[Number(remainder)] + result;
    num = num / base;
  }
  return result;
};
export const base62ToBigInt = (base62Str: string): bigint => {
    let result = 0n;
    const base = 62n;
    for (let i = 0; i < base62Str.length; i++) {
      const char = base62Str[i];
      const value = BigInt(BASE62_ALPHABET.indexOf(char)); 
      if (value === -1n) {
        throw new Error(`Invalid character '${char}' in Base62 string.`);
      }
  
      result = result * base + value; 
    }
    return result;
  };