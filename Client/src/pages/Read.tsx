import { useEffect, useState } from "react";
import { useParams } from "react-router-dom";
import ReadPasteBody from "@/components/ReadPaste/Body";
import ReadPasteHeader from "@/components/ReadPaste/Header";
import { PasteService } from "@/services/PasteService";
import { GetPaste } from "@/models/Paste/Response/GetPasteResponse";
import { decryptData } from "@/utils/crypto";
import { base62ToBigInt } from "@/utils/bigint-base62";
import { bigIntToUUID } from "@/utils/bigint-uuid";
import GetUserKeyAlert from "@/components/ReadPaste/GetUserKeyAlert";
import { toast } from "@/hooks/use-toast";
import NotFoundPage from "./NotFound";
const ReadPaste = () => {
  const { id, key, iv } = useParams();
  const [secretkey, setSecretKey] = useState<string>(key || "");
  const [ivkey, setIvKey] = useState<string>(iv || "");
  const [exist, setExist] = useState(false);
  const [openAltert, setOpenAlert] = useState(false);
  const [TpasteData, setPasteData] = useState<GetPaste>();
  const [DecryptedPasteData, setDecryptedPasteData] = useState<GetPaste>();
  const decryptAllData = (pasteData = TpasteData) => {
    if (pasteData != null) {
      if (
        decryptData(ivkey || "", secretkey || "", pasteData.signature) !=
        "incognito"
      ) {
        if (secretkey != "") {
          toast({
            variant: "destructive",
            title: "Wrong Or Missing Keys?",
            description: "Please Set A Valid Secret Key And IV Key",
          });
        }
        setOpenAlert(true);
      } else {
        const DecryptedPaste: GetPaste = {
          title: decryptData(ivkey, secretkey, pasteData.title),
          content: decryptData(ivkey, secretkey, pasteData.content),
          signature: "",
          syntax: decryptData(ivkey, secretkey, pasteData.syntax),
          expire: pasteData.expire,
          views: pasteData.views,
        };
        setDecryptedPasteData(DecryptedPaste);
        setOpenAlert(false);
        setExist(true);
      }
    }
  };
  useEffect(() => {
    const fetchData = async () => {
      try {
        if (id != null) {
          const pasteDataRes: GetPaste | null = await PasteService.getPasteById(
            bigIntToUUID(base62ToBigInt(id))
          );
          if (pasteDataRes) {
            setPasteData(pasteDataRes);
            decryptAllData(pasteDataRes);
            setExist(true);
          } else {
            setExist(false);
          }
        }
      } catch (error) {
        console.error("Error fetching paste:", error);
        setExist(false);
      }
    };

    fetchData();
  }, [id]);

  return (
    <div>
      <GetUserKeyAlert
        IvKey={ivkey}
        SecretKey={secretkey}
        SetSecretKey={setSecretKey}
        SetIvKey={setIvKey}
        Open={openAltert}
        OnDecryptClick={() => {
          decryptAllData();
        }}
      />
      {exist ? (
        DecryptedPasteData ? (
          <div className="m-4 flex justify-center">
            <div className="2xl:max-w-[1468.8px] w-full">
              <ReadPasteHeader
                title={
                  DecryptedPasteData.title != ""
                    ? DecryptedPasteData.title
                    : "No Title"
                }
                expiration={DecryptedPasteData.expire}
                views={DecryptedPasteData.views}
              />
              <ReadPasteBody
                syntax={DecryptedPasteData.syntax}
                content={DecryptedPasteData.content}
              />
            </div>
          </div>
        ) : (
          <></>
        )
      ) : (
        <NotFoundPage />
      )}
    </div>
  );
};
export default ReadPaste;
