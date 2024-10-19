import { Input } from "../../ui/input";
import { Label } from "../../ui/label";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@/components/ui/tooltip";
interface InputProps {
  InputLabelText: string;
  InputPlaceHolder: string;
  ToolTipText: string;
  InputValue: string;
  setInputText: (event: string) => void;
}

const InputWithLabel: React.FC<InputProps> = ({
  InputLabelText,
  InputPlaceHolder,
  ToolTipText,
  InputValue,
  setInputText,
}) => {
  return (
    <div className="flex justify-between items-end w-[15rem]">
      <TooltipProvider>
        <Tooltip>
          <TooltipTrigger asChild>
            <div className="grid w-60 items-center gap-1.5">
              <Label htmlFor="myLabel ">{InputLabelText}</Label>
              <Input
                onChange={(e)=>{setInputText(e.target.value)}}
                type="text"
                id="myLabel"
                placeholder={InputPlaceHolder}
                maxLength={32}
                value={InputValue}
              />
            </div>
          </TooltipTrigger>
          <TooltipContent>
            <p className="select-none">{ToolTipText}</p>
          </TooltipContent>
        </Tooltip>
      </TooltipProvider>
    </div>
  );
};

export default InputWithLabel;
