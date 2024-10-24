import { Icons } from "@/components/icons";
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogTrigger,
} from "@/components/ui/alert-dialog";
import { Button } from "@/components/ui/button";
import { Separator } from "@/components/ui/separator";
import { DEFAULT_KEY } from "@/utils/crypto";
import { forwardRef, useState } from "react";
interface PasteLinksAlertProps
  extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  SecretKey: string;
  PasteId: string;
  IvKey: string;
}

const PasteLinksAlert = forwardRef<HTMLButtonElement, PasteLinksAlertProps>(
  ({ SecretKey,PasteId ,IvKey}, ref) => {
    const [CopiedIndex,setCopiedIndex] = useState(-1);
    const copyToClipBoard = (Text: string,Index:number) => {
        setCopiedIndex(Index);
        navigator.clipboard.writeText(Text);
      };
    return (
        <AlertDialog>
        <AlertDialogTrigger asChild>
          <Button className="hidden" ref={ref}></Button>
        </AlertDialogTrigger>
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>Copy Your Paste links</AlertDialogTitle>
            <AlertDialogDescription>
              Without Secret Key and IV Key
            </AlertDialogDescription>
            <div className="flex gap-1 justify-between items-center bg-slate-900 w-[420px] p-0.5 rounded-md pl-3">
              <div className="pr-1.5">
                <Icons.link className="w-5 [&>path]:fill-slate-300" />
              </div>
              <Separator orientation="vertical" />
              <div className="text-neutral-400 w-full overflow-hidden h-6 whitespace-nowrap">
                http://localhost:5173/{PasteId}
              </div>
              <Button variant="secondary" onClick={()=>copyToClipBoard(`http://localhost:5173/${PasteId}`,0)}>{CopiedIndex!=0?"Copy":"Copied"}</Button>
            </div>

            {SecretKey != "" ? (
              <>
                <AlertDialogDescription>
                  With Secret Key and Without IV Key
                </AlertDialogDescription>
                <div className="flex gap-1 justify-between items-center bg-slate-900 w-[420px] p-0.5 rounded-md pl-3">
                  <div className="pr-1.5">
                    <Icons.link className="w-5 [&>path]:fill-slate-300" />
                  </div>
                  <Separator orientation="vertical" />
                  <div className="text-neutral-400 w-full overflow-hidden h-6 whitespace-nowrap">
                    http://localhost:5173/{PasteId}/{SecretKey}
                  </div>
                  <Button variant="secondary" onClick={()=>copyToClipBoard(`http://localhost:5173/${PasteId}/${SecretKey}`,1)}>{CopiedIndex!=1?"Copy":"Copied"}</Button>
                </div>
              </>
            ) : (
              <></>
            )}
            {IvKey != "" ? (
              <>
                <AlertDialogDescription>
                  With Secret Key and IV Key
                </AlertDialogDescription>
                <div className="flex gap-1 justify-between items-center bg-slate-900 w-[420px] p-0.5 rounded-md pl-3">
                  <div className="pr-1.5">
                    <Icons.link className="w-5 [&>path]:fill-slate-300" />
                  </div>
                  <Separator orientation="vertical" />
                  <div className="text-neutral-400 w-full overflow-hidden h-6 whitespace-nowrap">
                    http://localhost:5173/{PasteId}/{SecretKey!=""?SecretKey:DEFAULT_KEY}/{IvKey}
                  </div>
                  <Button variant="secondary" onClick={()=>copyToClipBoard(`http://localhost:5173/${PasteId}/${SecretKey!=""?SecretKey:DEFAULT_KEY}/${IvKey}`,2)}>{CopiedIndex!=2?"Copy":"Copied"}</Button>
                </div>
              </>
            ) : (
              <></>
            )}
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogAction>Done</AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
    );
  }
);

export default PasteLinksAlert;
