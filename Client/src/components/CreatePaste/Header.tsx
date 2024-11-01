import BurnAfterReadSwitch from "./Header/BurnAfterReadSwitch";
import InputWithGenerator from "./Header/InputWithGenerator";
import InputWithLabel from "./Header/InputWithLabel";
import ExpireWithLabel from "./Header/ExpireWithLabel";
import { SyntaxBox } from "./Header/SyntaxBox";
interface CreatePasteHeaderProps {
  burn: boolean;
  setBurn: (event: boolean) => void;
  syntax: string;
  setSyntax: (event: string) => void;
  secretkey: string;
  setSecretKey: (event: string) => void;
  ivkey: string;
  setIvKey: (event: string) => void;
  title: string;
  setTitle: (event: string) => void;
  setExpiration: (event: number) => void;
}
const CreatePasteHeader: React.FC<CreatePasteHeaderProps> = ({
  burn,
  setBurn,
  syntax,
  setSyntax,
  secretkey,
  setSecretKey,
  ivkey,
  setIvKey,
  title,
  setTitle,
  setExpiration,
}) => {
  return (
    <div className="flex w-screen md:w-auto flex-col 2xl:flex-row gap-3 justify-between 2xl:items-center items-start 2xl:h-[102px] pb-4">
      <div className="flex flex-col pl-3 md:pl-0 gap-5  md:flex-row justify-between md:w-[670px] md:gap-[4.5rem] 2xl:gap-0">
        <InputWithLabel
          InputLabelText="Title "
          InputPlaceHolder="Your Title (Optional)"
          ToolTipText="Paste Title"
          InputValue={title}
          setInputText={setTitle}
        />
        <SyntaxBox value={syntax} setValue={setSyntax} />
        <ExpireWithLabel setExpiration={setExpiration} />
      </div>
      <div className="flex flex-col pl-3 md:pl-0 gap-5 md:gap-0 md:flex-row justify-between md:w-[776.8px] w-screen 2xl:ml-5 ml-0">
          <InputWithGenerator
            InputLabelText="Secret Key"
            InputPlaceHolder="Private Secret Key"
            ToolTipText="Random Secret Key"
            InputLength={32}
            Secret={secretkey}
            setSecretKey={setSecretKey}
          />
          <InputWithGenerator
            InputLabelText="IV Key "
            InputPlaceHolder="Private IV Key (Optional)"
            ToolTipText="Random IV Key"
            InputLength={16}
            Secret={ivkey}
            setSecretKey={setIvKey}
          />
          <div className="mt-0 md:mt-7">
              <BurnAfterReadSwitch burn={burn} onBurnClick={()=>setBurn(burn?false:true)} />
          </div>
        </div>
    </div>
  );
};
export default CreatePasteHeader;
