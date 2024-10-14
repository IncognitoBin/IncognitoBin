import CryptoJS from "crypto-js";
const DEFAULT_KEY = "iYCcUmX4Xb6m2jQ6s8nXHKhJkK29EeOv";
const DEFAULT_IV = "hlx8B6w7z31nv935";

function padIfTooShort(
  value: string,
  requiredLength: number,
  defaultValue: string
): string {
  if (value.length < requiredLength) {
    return value + defaultValue.slice(0, requiredLength - value.length);
  }
  return value;
}

export function encryptData(
  ivStr: string,
  secretKey: string,
  data: string
): string {
  const iv = CryptoJS.enc.Utf8.parse(padIfTooShort(ivStr, 16, DEFAULT_IV));
  const key = CryptoJS.enc.Utf8.parse(
    padIfTooShort(secretKey, 32, DEFAULT_KEY)
  );

  const ciphertext = CryptoJS.AES.encrypt(data, key, {
    iv: iv,
    mode: CryptoJS.mode.CBC,
    padding: CryptoJS.pad.Pkcs7,
  }).toString();

  return ciphertext;
}

export function decryptData(
    ivStr: string,
    secretKey: string,
    data: string
): string {
  const iv = CryptoJS.enc.Utf8.parse(padIfTooShort(ivStr, 16, DEFAULT_IV));
  const key = CryptoJS.enc.Utf8.parse(
      padIfTooShort(secretKey, 32, DEFAULT_KEY)
  );
  const originalData = CryptoJS.AES.decrypt(data, key, {
    iv: iv,
    mode: CryptoJS.mode.CBC,
    padding: CryptoJS.pad.Pkcs7,
  }).toString(CryptoJS.enc.Utf8);
  return originalData;
}
