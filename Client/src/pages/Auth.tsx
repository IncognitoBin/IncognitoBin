import * as React from "react";
import { useEffect, useState } from "react";
import { UserService } from "@/services/UserService";
import { NewUserResponse } from "@/models/User/Response/NewUserResponse";
import { bigIntToUUID, uuidToBigInt } from "@/utils/bigint-uuid";
import NewUserAlert from "@/components/Auth/NewUserAlert";
import { toast } from "@/hooks/use-toast";
import { UserLoginRequest } from "@/models/User/Request/UserLoginRequest";
import { UserLoginResponse } from "@/models/User/Response/UserLoginResponse";
import { useNavigate, useParams } from "react-router-dom";
import AuthSection from "@/components/Auth/AuthSection";

const AuthPage = () => {
  const [accountNumber, setAccountNumber] = useState("");
  const { id } = useParams();
  useEffect(() => {
    if(id){
      Login(id);
    }
  }, []);
  
  const [isLogin, setisLogin] = React.useState<boolean>(false);
  const [isCreate, setisCreate] = React.useState<boolean>(false);
  const [OpenAlert, setOpenAlert] = React.useState<boolean>(false);
  
  const navigate = useNavigate();

  async function Login(accNumber:string) {
    setisLogin(true);
    try {
      const UserLognReq: UserLoginRequest = {
        id: bigIntToUUID(BigInt(accNumber.replace(/\s+/g, ""))),
      };
      const NewUserRes: UserLoginResponse | null = await UserService.Login(UserLognReq);
  
      if (NewUserRes) {
        UserService.setUserToken(NewUserRes.token);
        navigate('/Create');
      } 
    } catch (error: any) {
      console.error("Error logging in:", error);
      if (error.response?.status === 404) {
        toast({
          variant: "destructive",
          title: "Invalid Account Number",
          description: "The account number provided is invalid.",
          action: <></>,
        });
      } else {
        toast({
          variant: "destructive",
          title: "Something Went Wrong",
          description: "Please Try again later.",
          action: <></>,
        });
      }
    } finally {
      setisLogin(false);
    }
  }

  async function onLoginClick() {
    Login(accountNumber);
  }

  async function onCreate() {
    setisCreate(true);
    try {
      const NewUserRes: NewUserResponse | null = await UserService.New();
      if (NewUserRes) {
        setAccountNumber(
          formatAccountNumber(uuidToBigInt(NewUserRes.id).toString())
        );
        setOpenAlert(true);
      }
    } catch (error) {
      console.error("Error fetching paste:", error);
      toast({
        variant: "destructive",
        title: "Something Went Wrong",
        description: "Please Try again later.",
        action: <></>,
      });
    } finally {
      setisCreate(false);
    }
  }

  const formatAccountNumber = (value: string) => {
    value = value.replace(/\D/g, "");
    return value.replace(/(.{6})/g, "$1 ").trim();
  };

  const handleNumberChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const formattedValue = formatAccountNumber(e.target.value);
    setAccountNumber(formattedValue);
  };

  const OnDoneClick = () => {
    setOpenAlert(false);
  };

  return (
    <>
      <NewUserAlert
        UserId={accountNumber}
        Open={OpenAlert}
        OnDoneClick={OnDoneClick}
      />
      <AuthSection onLoginClick={onLoginClick} onCreateClick={onCreate} isLogin={isLogin} isCreate={isCreate} accountNumber={accountNumber} handleAccountNumberChange={handleNumberChange}/>
    </>
  );
};

export default AuthPage;
