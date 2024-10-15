export const bigIntToUUID = (bigIntValue: bigint): string => {
    let hexString = bigIntValue.toString(16).padStart(32, '0');
    const uuid = `${hexString.substring(0, 8)}-${hexString.substring(8, 12)}-${hexString.substring(12, 16)}-${hexString.substring(16, 20)}-${hexString.substring(20)}`;
    return uuid;
  };

export const uuidToBigInt = (uuid: string): bigint => {
    const hexString = uuid.replace(/-/g, '');
    return BigInt('0x' + hexString);
  };
