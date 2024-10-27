import { useEffect, useState } from "react";
import { PasteRow, columns } from "@/components/Pastes/columns";
import { DataTable } from "@/components/Pastes/data-table";
import { UserService } from "@/services/UserService";
import { PasteService } from "@/services/PasteService";
import { uuidToBigInt } from "@/utils/bigint-uuid";
import { BigInttoBase62 } from "@/utils/bigint-base62";
import { Button } from "@/components/ui/button";
import { useNavigate } from "react-router-dom";
import { toast } from "@/hooks/use-toast";

function convertToLargestUnit(seconds: number): string {
  const conversions = [
    { unit: "year", seconds: 31536000 },
    { unit: "month", seconds: 2628000 },
    { unit: "day", seconds: 86400 },
    { unit: "hour", seconds: 3600 },
    { unit: "minute", seconds: 60 },
    { unit: "second", seconds: 1 },
  ];
  for (const conversion of conversions) {
    const value = seconds / conversion.seconds;
    if (value >= 1) {
      const roundedValue = Math.floor(value);
      return `${roundedValue} ${conversion.unit}${roundedValue > 1 ? "s" : ""}`;
    }
  }
  return "0 seconds";
}

const MyPastes = () => {
  const navigate = useNavigate();
  const [data, setData] = useState<PasteRow[]>([]);
  const [selectedPastes, setSelectedPastes] = useState<string[]>([]);
  async function fetchData() {
    let PasteRowList: PasteRow[] = [];
    const userToken = UserService.getUserCookie();
    if(userToken){
      try {
        const AllPastes = await PasteService.getAllPastes(userToken);
        AllPastes.forEach((paste) => {
          let NewPasteRow: PasteRow = {
            id: BigInttoBase62(uuidToBigInt(paste.id)),
            expire: convertToLargestUnit(paste.expire || 0),
            burn: paste.burn ? "Yes" : "No",
            views: paste.views,
          };
          PasteRowList.push(NewPasteRow);
        });
        setData(PasteRowList);
      } catch (error:any) {
        if(error?.status==401){
          UserService.setUserToken("");
          navigate("/Auth");
        }else{
          toast({
            variant: "destructive",
            title: "Something Went Wrong",
            description: "Please Try again later.",
            action: <></>,
          });
        }
      }
    }else{
      navigate("/Auth");
    }
  }
  useEffect(() => {
    fetchData();
  }, []);
  const DeleteAction = async () => {
    const userToken = UserService.getUserCookie();
    if (userToken) {
      for (const paste_id of selectedPastes) {
        await PasteService.deletePaste(paste_id, userToken);
      }
      toast({
        variant: "default",
        title: "Operation Done Successfully!",
        description: "Maybe? IDK",
        action: <></>,
      });
      await fetchData();
    }
  };
  
  return (
    <div className="w-full flex justify-center">
      <div className="w-[50rem] flex flex-col gap-4 m-3">
        <div>
          <Button
            className=""
            variant={"destructive"}
            onClick={DeleteAction}
            disabled={selectedPastes.length > 0 ? false : true}
          >
            Delete
          </Button>
        </div>
        <div>
          <DataTable
            columns={columns}
            data={data}
            setSelectedUuid={setSelectedPastes}
          />
        </div>
      </div>
    </div>
  );
};

export default MyPastes;
