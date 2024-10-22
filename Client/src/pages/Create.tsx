import CreatePasteBody from "@/components/CreatePaste/Body";
import CreatePasteHeader from "@/components/CreatePaste/Header";
import { useState } from "react";

const CreatePaste = () => {
    // Header
  const [burn, setBurn] = useState(false);
  const [syntax, setSyntax] = useState("");
  const [secretkey, setSecretKey] = useState("");
  const [ivkey, setIvKey] = useState("");
  const [title, setTitle] = useState("");
  const [expiration, setExpiration] = useState(300);
  // Body
  const [content, setContent] = useState("");

  return (
    <div className="m-4 flex flex-col items-center">
      <CreatePasteHeader
        burn={burn}
        setBurn={setBurn}
        syntax={syntax}
        setSyntax={setSyntax}
        secretkey={secretkey}
        setSecretKey={setSecretKey}
        ivkey={ivkey}
        setIvKey={setIvKey}
        title={title}
        setTitle={setTitle}
        setExpiration={setExpiration}
      />
      <CreatePasteBody Syntax={syntax} Content={content} setContent={setContent} Create={()=> {} } />
    </div>
  );
};
export default CreatePaste;