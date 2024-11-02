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
import { forwardRef, useState } from "react";
import { Alert, AlertDescription, AlertTitle } from "@/components/ui/alert";
import { TriangleAlert } from "lucide-react";

interface PasteLinksAlertProps
  extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  UserId: string;
  Open: boolean;
  OnDoneClick: () => void;
}
const downloadTextFile = (content: string) => {
  const blob = new Blob([content], { type: "text/plain" });
  const url = URL.createObjectURL(blob);
  const link = document.createElement("a");
  link.href = url;
  link.download = "IncognitoBin_account.txt";
  link.click();
  URL.revokeObjectURL(url);
};
const NewUserAlert = forwardRef<HTMLButtonElement, PasteLinksAlertProps>(
  ({ UserId, Open, OnDoneClick }) => {
    const [CopiedIndex, setCopiedIndex] = useState(-1);
    const copyToClipBoard = (Text: string, Index: number) => {
      setCopiedIndex(Index);
      navigator.clipboard.writeText(Text);
    };
    return (
      <AlertDialog open={Open}>
        <AlertDialogTrigger asChild>
          <Button className="hidden"></Button>
        </AlertDialogTrigger>
        <AlertDialogContent className="max-w-[650px] p-5 px-1.5 rounded-lg sm:p-[1.5rem]">
          <AlertDialogHeader>
            <AlertDialogTitle>Account Created !</AlertDialogTitle>
            <AlertDialogDescription>
              Congrats! Here's your account number:
            </AlertDialogDescription>
            <div className="flex gap-1 justify-between items-center bg-slate-900 sm:w-[600px] p-0.5 rounded-md pl-3 w-screen">
              <div className="text-neutral-400 w-full overflow-hidden h-6 whitespace-nowrap">
                {UserId}
              </div>
              <Button
                variant="secondary"
                onClick={() => copyToClipBoard(`${UserId}`, 0)}
                className="flex gap-2 "
              >
                {CopiedIndex != 0 ? (
                  <Icons.clipboard className="w-5 [&>path]:fill-current" />
                ) : (
                  <Icons.done className="w-4 [&>path]:fill-current" />
                )}
                {CopiedIndex != 0 ? "Copy" : "Copied"}
              </Button>
              <Button
                variant="secondary"
                onClick={() => downloadTextFile(`${UserId}`)}
                className="flex gap-2 "
              >
                <Icons.download className="w-5 [&>path]:fill-current" />
                Download
              </Button>
            </div>
            <AlertDialogDescription>Quick Login Link</AlertDialogDescription>
            <div className="flex gap-1 justify-between items-center bg-slate-900 sm:w-[600px] p-0.5 rounded-md pl-3 w-screen">
              <div className="text-neutral-400 w-full overflow-hidden h-6 whitespace-nowrap">
              {window.location.origin}/Auth/{UserId.replace(/\s+/g, "")}
              </div>
              <Button
                variant="secondary"
                onClick={() => copyToClipBoard(`${window.location.origin}/Auth/${UserId.replace(/\s+/g, "")}`, 1)}
                className="flex gap-2 "
              >
                {CopiedIndex != 1 ? (
                  <Icons.clipboard className="w-5 [&>path]:fill-current" />
                ) : (
                  <Icons.done className="w-4 [&>path]:fill-current" />
                )}
                {CopiedIndex != 1 ? "Copy" : "Copied"}
              </Button>
            </div>
            
            <Alert variant={"orange"}>
              <TriangleAlert className="h-4 w-4" />
              <AlertTitle>Notice</AlertTitle>
              <AlertDescription>
                Don't lose it! It's your only key to access our service — no
                email, no username. If you lose it, you won’t be able to recover
                your account.
              </AlertDescription>
            </Alert>
          </AlertDialogHeader>

          <AlertDialogFooter>
            <AlertDialogAction onClick={OnDoneClick}>Done</AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
    );
  }
);

export default NewUserAlert;
