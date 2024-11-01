import { UserService } from "@/services/UserService";
import MiniLogo from "../assets/miniLogo.webp";
import { useEffect, useState } from "react";
import { Link, useNavigate } from "react-router-dom";
import { Button } from "@/components/ui/button";

const Header = () => {
  const navigate = useNavigate();
  const logoutHandler = async () => {
    try {
      const userToken = UserService.getUserCookie() || "";
      await UserService.LogOut(userToken);
      UserService.setUserToken("");
      setConnected(false);
      navigate("/Auth");
    } catch (error) {
      console.error("Error logging out:", error);
    }
  };

  const [connected, setConnected] = useState(false);
  const [prevToken, setPrevToken] = useState("");

  useEffect(() => {
    const checkTokenChange = () => {
      const currentToken = UserService.getUserCookie();
      if (currentToken && currentToken !== prevToken) {
        if (currentToken && currentToken !== "") {
          setConnected(true);
        } else {
          setConnected(false);
        }
        setPrevToken(currentToken);
      }
    };
    const intervalId = setInterval(checkTokenChange, 250);
    return () => clearInterval(intervalId);
  }, [prevToken]);

  return (
    <div className="border-b-2 p-3 flex justify-center">
      <div className="flex items-center justify-between gap-2 w-[1500px]">
        <Link to="/Create">
          <div className="flex gap-2 items-center">
            <img className="w-[3.5rem]" src={MiniLogo} alt="Mini Logo" />
            <div className="flex flex-col items-center">
              <div className="font-semibold text-xl text-slate-200">
                IncognitoBin
              </div>
            </div>
          </div>
        </Link>
        <div className="flex gap-6 items-center">
          {connected ? (
            <>
              <Link
                className="text-sm sm:text-base font-semibold text-slate-200 cursor-pointer hover:underline"
                to="/Me"
              >
                My Pastes
              </Link>
              <div
                className="text-sm sm:text-base font-semibold text-slate-200 cursor-pointer hover:underline"
                onClick={logoutHandler}
              >
                Log Out
              </div>
            </>
          ) : (
            <Link to="/Auth">
              <Button>Account</Button>
            </Link>
          )}
        </div>
      </div>
    </div>
  );
};

export default Header;
