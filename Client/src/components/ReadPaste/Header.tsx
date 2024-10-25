
import { Icons } from "@/components/icons";
interface ReadPasteHeaderProps {
  title: string;
  expiration: number;
  views: number;
}
const ReadPasteHeader: React.FC<ReadPasteHeaderProps> = ({
  title,
  expiration,
  views,
}) => {
  const SetExpirationDate = (seconds: number) => {
    const currentDate = new Date();
    currentDate.setSeconds(currentDate.getSeconds() + seconds);
    const hours = currentDate.getHours().toString().padStart(2, "0");
    const minutes = currentDate.getMinutes().toString().padStart(2, "0");
    const day = currentDate.getDate();
    const month = currentDate.toLocaleString("default", { month: "long" });
    const year = currentDate.getFullYear();
    return `${hours}:${minutes} ${day} ${month} ${year}`;
  };
  return (
    <>
      <div className="text-xl p-1">{title}</div>
      <div className="flex justify-between gap-3 opacity-80 py-2 px-1 text-sm">
        <div className="flex items-center gap-1">
          <Icons.date className=" w-[21px] [&>path]:fill-current" />
          {SetExpirationDate(expiration)}
        </div>
        <div className="flex items-center gap-1">
          <Icons.view className=" w-[21px] [&>path]:fill-current" />
          <div>{views}</div>
        </div>
      </div>
    </>
  );
};
export default ReadPasteHeader;
