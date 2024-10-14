import { useState } from 'react';
import './App.css'
import { encryptData,decryptData } from './utils/crypto';
import { PasteService } from './services/PasteService';
import { CreatePaste } from './models/Paste/Request/CreatePasteRequest';

function App() {
  const [text, setText] = useState('');
  const [key, setKey] = useState('');
  const [iv, setIV] = useState('');
  const [encryptedText, setEncryptedText] = useState('');
  const [decryptedText, setDecryptedText] = useState('');

  return (
    <div>
      <h1>Key + IV</h1>
      <input
        type="text"
        value={key}
        onChange={(e) => setKey(e.target.value)}
        maxLength={32}
        placeholder="Enter Key"
      />
      <input
        type="text"
        value={iv}
        onChange={(e) => setIV(e.target.value)}
        maxLength={16}
        placeholder="Enter IV"
      />
      <h1>AES-256</h1>
      <input
        type="text"
        value={text}
        onChange={(e) => setText(e.target.value)}
        placeholder="Text"
      />
      <button onClick={()=>{setEncryptedText(encryptData(iv,key,text))}}>Encrypt</button>

      <h2>Encrypted Text:</h2>
      <p>{encryptedText}</p>

      <button onClick={()=>{setDecryptedText(decryptData(iv,key,encryptedText))}}>Decrypt</button>

      <h2>Decrypted Text:</h2>
      <p>{decryptedText}</p>
    </div>
  );
}

export default App
