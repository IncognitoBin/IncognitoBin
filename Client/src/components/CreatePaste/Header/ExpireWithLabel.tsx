import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Label } from "../../ui/label";
interface ExpirationProps {
  setExpiration: (value: number) => void;
}
const ExpireWithLabel: React.FC<ExpirationProps> = ({ setExpiration }) => {
  return (
    <div className="grid items-center gap-1.5">
      <TooltipProvider>
        <Tooltip>
          <TooltipTrigger asChild>
            <Label htmlFor="expire">Expire After</Label>
          </TooltipTrigger>
          <div id="expire" className="inline-block">
            <Select
              defaultValue="300"
              onValueChange={(v) => {
                setExpiration(Number(v));
              }}
            >
              <SelectTrigger className="w-[150px]">
                <SelectValue placeholder="Select Paste Expiration" />
              </SelectTrigger>
              <SelectContent>
                <SelectGroup className="[&>*]:cursor-pointer">
                  <SelectItem value="60">1 Minute</SelectItem>
                  <SelectItem value="300">5 Minutes</SelectItem>
                  <SelectItem value="600">10 Minutes</SelectItem>
                  <SelectItem value="1800">30 Minutes</SelectItem>
                  <SelectItem value="3600">1 Hour</SelectItem>
                  <SelectItem value="86400">1 Day</SelectItem>
                  <SelectItem value="604800">1 Week</SelectItem>
                  <SelectItem value="1209600">2 Weeks</SelectItem>
                  <SelectItem value="2592000">1 Month</SelectItem>
                  <SelectItem value="7776000">3 Months</SelectItem>
                  <SelectItem value="31536000">1 Year</SelectItem>
                </SelectGroup>
              </SelectContent>
            </Select>
          </div>
          <TooltipContent>
            <p className="select-none">
              The Paste Will Expire After A Duration
            </p>
          </TooltipContent>
        </Tooltip>
      </TooltipProvider>
    </div>
  );
};
export default ExpireWithLabel;