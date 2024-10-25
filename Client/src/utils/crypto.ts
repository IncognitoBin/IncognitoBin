import CryptoJS from "crypto-js";
export const DEFAULT_KEY = "iYCcUmX4Xb6m2jQ6s8nXHKhJkK29EeOv";
const DEFAULT_IV = "hlx8B6w7z31nv935";
export function generateRandomKey(length: number): string {
  const characters = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
  let result = '';
  
  for (let i = 0; i < length; i++) {
    const randomIndex = Math.floor(Math.random() * characters.length);
    result += characters[randomIndex];
  }

  return result;
}
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
  if(data.trim().length > 0){
    const ciphertext = CryptoJS.AES.encrypt(data, key, {
      iv: iv,
      mode: CryptoJS.mode.CBC,
      padding: CryptoJS.pad.Pkcs7,
    }).toString();
    return ciphertext;
  }
  return ""
}

export function decryptData(
  ivStr: string,
  secretKey: string,
  data: string
): string {
  if (!data) {
    return ""; 
  }

  try {
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
  } catch (e) {
    console.error("Error during decryption:", e);
    return ""; // Return an empty string or handle the error case as needed
  }
}
