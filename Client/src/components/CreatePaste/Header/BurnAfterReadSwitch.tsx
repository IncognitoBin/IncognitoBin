import { Switch } from "../../ui/switch";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@/components/ui/tooltip";
interface BurnAfterReadSwitchProps {
    burn: boolean;
    onBurnClick: () => void;
  }
  const BurnAfterReadSwitch: React.FC<BurnAfterReadSwitchProps> = ({ burn, onBurnClick }) => {
    return (
      <TooltipProvider>
        <Tooltip>
          <TooltipTrigger asChild>
            <div className="flex justify-between w-[9.4rem] items-center cursor-pointer">
              <Switch onClick={onBurnClick} checked={burn} />
              <div
                onClick={onBurnClick}
                className="text-sm font-medium leading-none select-none"
              >
                Burn After Read
              </div>
            </div>
          </TooltipTrigger>
          <TooltipContent>
            <p className="select-none">Delete After First View</p>
          </TooltipContent>
        </Tooltip>
      </TooltipProvider>
    );
  };
  
  export default BurnAfterReadSwitch;