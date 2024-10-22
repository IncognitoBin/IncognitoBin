import { useEffect, useRef, useState } from "react";
import { Icons } from "../icons";
import { Prism as SyntaxHighlighter } from "react-syntax-highlighter";
import {
  materialDark,
  materialLight,
} from "react-syntax-highlighter/dist/cjs/styles/prism";
import { cn } from "@/lib/utils";
import { Button } from "../ui/button";
import ContentArea from "./Body/ContentArea";
import ContentHeader from "./Body/ContentHeader";
interface CreatePasteBodyProps {
  Syntax: string;
  Content: string;
  setContent: (event: string) => void;
  Create: () => void;
}
const CreatePasteBody: React.FC<CreatePasteBodyProps> = ({
  Content,
  setContent,
  Syntax,
  Create,
}) => {
  const [tap, setTap] = useState(0);

  return (
    <div>
      <div className="bg-[var(--chal-item-bg)] rounded-xl border-[1px] border-slate-500/50 ">
        <ContentHeader CreateAction={Create} setTap={setTap} tap={tap} />
        <div className="w-[796px] 2xl:w-[1468.8px]">
          <ContentArea
            Syntax={Syntax}
            Content={Content}
            setContent={setContent}
            tap={tap}
          />
        </div>
      </div>
    </div>
  );
};
export default CreatePasteBody;
