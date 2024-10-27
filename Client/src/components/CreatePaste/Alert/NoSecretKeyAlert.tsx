import {
    AlertDialog,
    AlertDialogAction,
    AlertDialogCancel,
    AlertDialogContent,
    AlertDialogDescription,
    AlertDialogFooter,
    AlertDialogHeader,
    AlertDialogTitle,
    AlertDialogTrigger,
  } from "@/components/ui/alert-dialog";
import { Button } from "@/components/ui/button";
import { forwardRef } from "react";
interface ButtonComponentProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
    CreateNewPaste: () => void;
  }
  
const NoSecretKeyAlert  = forwardRef<HTMLButtonElement, ButtonComponentProps>(
    ({ CreateNewPaste }, ref) => {

    return (<AlertDialog>
        <AlertDialogTrigger asChild>
          <Button className="hidden" ref={ref}></Button>
        </AlertDialogTrigger>
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>Are you absolutely sure?</AlertDialogTitle>
            <AlertDialogDescription>
              You Didn't Set A Secret Key A Default One Will be selected
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>Cancel</AlertDialogCancel>
            <AlertDialogAction onClick={CreateNewPaste}>
              Continue
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>)
});

export default NoSecretKeyAlert;