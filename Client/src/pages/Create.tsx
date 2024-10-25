import CreatePasteBody from "@/components/CreatePaste/Body";
import CreatePasteHeader from "@/components/CreatePaste/Header";
import { toast } from "@/hooks/use-toast";
import { CreatePasteRequest } from "@/models/Paste/Request/CreatePasteRequest";
import { PasteService } from "@/services/PasteService";
import { UserService } from "@/services/UserService";
import { BigInttoBase62 } from "@/utils/bigint-base62";
import { uuidToBigInt } from "@/utils/bigint-uuid";
import {encryptData } from "@/utils/crypto";
import { useRef, useState } from "react";

import NoSecretKeyAlert from "@/components/CreatePaste/Alert/NoSecretKeyAlert";
import PasteLinksAlert from "@/components/CreatePaste/Alert/PasteLinksAlert";
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

  const onCreateClickRef = useRef<HTMLButtonElement | null>(null);
  const onConfirmedRed = useRef<HTMLButtonElement | null>(null);

  const [newPasteId,setNewPasteId] = useState("");
  const onCreateClick = async () => {
    if (secretkey.length == 0) {
      if (onCreateClickRef.current) {
        onCreateClickRef.current.click();
      }
    } else {
      await CreateNewPaste();
    }
  };
  const resetFields = () => {
    setBurn(false);
    setSyntax("");
    setSecretKey("");
    setIvKey("");
    setTitle("");
    setExpiration(300);
    setContent("");
  };
  const CreateNewPaste = async () => {
    const newPaste: CreatePasteRequest = {
      title: encryptData(ivkey, secretkey, title),
      content: encryptData(ivkey, secretkey, content),
      signature: encryptData(ivkey, secretkey, "incognito"),
      syntax: encryptData(ivkey, secretkey, syntax),
      expire: expiration,
      burn: burn,
    };
    try {
      var paste_info = await PasteService.createPaste(
        newPaste,
        UserService.getUserCookie() || ""
      );
      var paste_id = BigInttoBase62(uuidToBigInt(paste_info.id));
      if (onConfirmedRed.current) {
        onConfirmedRed.current.click();
      }
      setNewPasteId(paste_id);
    } catch (error: any) {
      const errorMessage =
        error.response?.data?.error || "An unexpected error occurred";
      toast({
        variant: "destructive",
        title: "Something Went Wrong",
        description: errorMessage,
        action: <></>,
      });
    }
  };
  return (
    <div className="m-4 flex flex-col items-center">
      <NoSecretKeyAlert ref={onCreateClickRef} CreateNewPaste={CreateNewPaste}/>
      <PasteLinksAlert ref={onConfirmedRed} SecretKey={secretkey} IvKey={ivkey} PasteId={newPasteId} onDoneClick={resetFields}/>
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
      <CreatePasteBody
        Syntax={syntax}
        Content={content}
        setContent={setContent}
        Create={onCreateClick}
      />
    </div>
  );
};
export default CreatePaste;
