import { Icons } from "@/components/icons";
import { Button } from "@/components/ui/button";
import { cn } from "@/lib/utils";

interface ContentAreaProps {
    CreateAction: () => void;
    setTap: (event: number) => void;
    tap: number;
  }
const ContentHeader: React.FC<ContentAreaProps> = ({
    CreateAction,
    setTap,
    tap,
  }) => {
    return (
        <div className="rounded-t-xl text-[13px] flex justify-between  items-center h-8  py-5 border-b-2 mb-0.5">
          <div className="flex items-center [&>div:hover]:opacity-100 [&>div:hover]:bg-[var(--chal-itm-sktn-bg)] [&>div]:p-1.5 [&>div]:text-center [&>div]:rounded-md [&>div]:cursor-pointer [&>div]:ml-1">
            <div
              className={cn("flex gap-1 items-center", {
                "opacity-50": tap !== 0,
              })}
              onClick={() => setTap(0)}
            >
              <Icons.editor className="w-5 [&>path]:fill-sky-300 [&>path]:drop-shadow-[0_5px_5px_rgba(51,111,152,1)] [&>path]:stroke-sky-300 [&>path]:stroke-[15px]	" />
              Editor
            </div>
            <div
              className={cn("flex gap-1 items-center", {
                "opacity-50": tap !== 1,
              })}
              onClick={() => {
                setTap(1);
              }}
            >
              <Icons.view className="w-5 h-5 [&>path]:fill-purple-400 [&>path]:stroke-[15px] [&>path]:stroke-purple-400" />
              Preview
            </div>
          </div>
          <div className="flex items-center justify-between [&>div:hover]:opacity-100 [&>div:hover]:bg-[var(--chal-itm-sktn-bg)] [&>div]:p-1.5 [&>div]:text-center [&>div]:rounded-md [&>div]:cursor-pointer [&>div]:ml-1">
            <div className={cn("flex gap-1 items-center")}>
              <Button onClick={CreateAction} className="h-8 w-14 text-sm border-[1px] border-current" variant={"ghost"}>
                Create
              </Button>
            </div>
          </div>
        </div>
    )
}
export default ContentHeader;