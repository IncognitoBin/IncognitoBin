import { generateRandomKey } from "@/utils/crypto";
import { Icons } from "../../icons";
import { Button } from "../../ui/button";
import { Input } from "../../ui/input";
import { Label } from "../../ui/label";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@/components/ui/tooltip";
interface InputGenProps {
    InputLabelText: string;
    InputPlaceHolder: string;
    ToolTipText: string;
    InputLength: number;
    Secret: string;
    setSecretKey: (event: string) => void;
  }
  
  const InputWithGenerator: React.FC<InputGenProps> = ({ InputLabelText,InputPlaceHolder,ToolTipText,InputLength,Secret, setSecretKey }) => {
    const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const value = e.target.value;
        const allowedPattern = /^[a-zA-Z0-9]*$/;
        if (allowedPattern.test(value)) {
            setSecretKey(value);
        }
      };
    return (
        <div className="flex justify-between items-end w-[17.8rem]">
        <div className="grid w-60 items-center gap-1.5">
          <Label htmlFor="SecKey">{InputLabelText}</Label>
          <Input
          onChange={handleInputChange}
          type="text"
            id="SecKey"
            placeholder={InputPlaceHolder}
            maxLength={InputLength}
            value={Secret}
          />
        </div>
        <TooltipProvider>
          <Tooltip>
            <TooltipTrigger asChild>
              <Button className="rounded-full p-[0.47rem] [&:hover>svg>path]:fill-slate-100 hover:bg-slate-900 border-2" onClick={()=> {setSecretKey(generateRandomKey(InputLength))}}><Icons.random className="h-[1.3rem] [&>path]:fill-background" /></Button>
            </TooltipTrigger>
            <TooltipContent>
              <p className="select-none">{ToolTipText}</p>
            </TooltipContent>
          </Tooltip>
        </TooltipProvider>
      </div>
    );
  };
  
  export default InputWithGenerator;