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
import { forwardRef } from "react";
import { Input } from "../ui/input";
import { Button } from "../ui/button";
interface GetUserKeyAlertProps
  extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  SecretKey: string;
  IvKey: string;
  Open: boolean;
  SetSecretKey: (event: string) => void;
  SetIvKey: (event: string) => void;
  OnDecryptClick: () => void;
}

const GetUserKeyAlert = forwardRef<HTMLButtonElement, GetUserKeyAlertProps>(
  ({ SecretKey, IvKey, Open, SetSecretKey, SetIvKey, OnDecryptClick },ref) => {
    return (
      <AlertDialog open={Open}>
        <AlertDialogTrigger asChild>
          <Button className="hidden" ref={ref}></Button>
        </AlertDialogTrigger>
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>Verification </AlertDialogTitle>
            <AlertDialogDescription>Secret Key</AlertDialogDescription>
            <Input
              value={SecretKey}
              onChange={(e) => {
                SetSecretKey(e.target.value);
              }}
              placeholder="Secret Key, if it is set"
            />
            <AlertDialogDescription>IV Key</AlertDialogDescription>
            <Input
              value={IvKey}
              onChange={(e) => {
                SetIvKey(e.target.value);
              }}
              placeholder="IV Key, if it is set"
            />
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogAction onClick={OnDecryptClick}>
              Decrypt
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
    );
  }
);

export default GetUserKeyAlert;
