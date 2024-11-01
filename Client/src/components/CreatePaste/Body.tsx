import { useState } from "react";

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
    <div className="w-screen md:w-[796px] 2xl:w-[1468.8px]">
      <div className="bg-[var(--chal-item-bg)] rounded-xl border-[1px] border-slate-500/50 ">
        <ContentHeader CreateAction={Create} setTap={setTap} tap={tap} />
        <div className="">
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
