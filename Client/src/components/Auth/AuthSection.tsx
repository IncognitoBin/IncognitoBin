import { cn } from "@/lib/utils";
import { Icons } from "../icons";
import { Button } from "../ui/button";
import { PasswordInput } from "./PasswordInput";
interface AuthSectionProps {
  onLoginClick: () => void;
  onCreateClick: () => void;
  isLogin: boolean;
  accountNumber: string;
  isCreate: boolean;
  handleAccountNumberChange: (
    event: React.ChangeEvent<HTMLInputElement>
  ) => void;
}

const AuthSection: React.FC<AuthSectionProps> = ({
  isLogin,
  isCreate,
  onCreateClick,
  onLoginClick,
  accountNumber,
  handleAccountNumberChange,
}) => {
  return (
    <div className="pt-16 sm:pt-32 ">
      <div className="mx-auto flex w-full flex-col justify-center space-y-6 sm:w-[450px] border-[1px] p-12 rounded-md">
        <div className="flex flex-col space-y-2 text-center">
          <h1 className="text-2xl font-semibold tracking-tight">
            Sign in to your account
          </h1>
        </div>
        <div className={cn("grid gap-6")}>
          <div>
            <div className="grid gap-3">
              <div className="grid gap-1">
                <PasswordInput
                  placeholder="Enter your account number"
                  className="text-[0.80rem]"
                  value={accountNumber}
                  onChange={handleAccountNumberChange}
                  disabled={isLogin || isCreate}
                />
              </div>
              <Button disabled={isLogin || isCreate} onClick={onLoginClick}>
                {isLogin && (
                  <Icons.spinner className="mr-2 h-4 w-4 animate-spin" />
                )}
                Sign in
              </Button>
            </div>
          </div>
          <div className="relative">
            <div className="absolute inset-0 flex items-center">
              <span className="w-full border-t" />
            </div>
            <div className="relative flex justify-center text-xs uppercase">
              <span className="bg-background px-2 text-muted-foreground">
                Or
              </span>
            </div>
          </div>
          <div className="flex flex-col space-y-2 text-center">
            <h1 className="text-2xl font-semibold tracking-tight">
              Get started with IncognitoBin
            </h1>
            <p className="text-sm text-muted-foreground">
              Start by generating a random account number.
            </p>
          </div>
          <Button
            variant="outline"
            type="button"
            disabled={isLogin || isCreate}
            onClick={onCreateClick}
          >
            {isCreate ? (
              <Icons.spinner className="mr-2 h-4 w-4 animate-spin" />
            ) : null}
            Create new account
          </Button>
        </div>
      </div>
    </div>
  );
};
export default AuthSection;
