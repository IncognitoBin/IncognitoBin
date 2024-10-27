import { Prism as SyntaxHighlighter } from "react-syntax-highlighter";
import {
  materialDark,
  materialLight,
} from "react-syntax-highlighter/dist/cjs/styles/prism";
import {
    Tooltip,
    TooltipContent,
    TooltipProvider,
    TooltipTrigger,
  } from "@/components/ui/tooltip";
  import { Icons } from "@/components/icons";
import { cn } from "@/lib/utils";
interface ReadPasteBodyProps {
    syntax:string,
    content:string
}
const ReadPasteBody : React.FC<ReadPasteBodyProps> = ({ syntax , content }) => {
    const customStyle = {
        margin: "0px",
        padding: "10px 0px 10px 10px",
        borderRadius: "0 0 10px 10px",
        border: "1px solid gray",
        "border-width": "0 1px 1px 1px",
      };
    return (
        <div className="">
          <div className="flex bg-[var(--submission-bar-bg)] justify-between pl-2 items-center border-gray-500 border-[1px] border-b-[var(--submission-bar-bg-line)] rounded-t-lg pr-0.5">
            <div className="opacity-85 text-[12px] font-light">
              {" "}
              {syntax.charAt(0).toUpperCase() + syntax.slice(1)}
            </div>
            <TooltipProvider>
              <Tooltip>
                <TooltipTrigger asChild>
                  <div>
                    <Icons.clipboard className="opacity-70 w-[21px] px-0.5 py-1 my-0.5 cursor-pointer hover:bg-gray-500 rounded [&>path]:fill-current" />
                  </div>
                </TooltipTrigger>
                <TooltipContent className="text-xs px-2 py-1 opacity-85">
                  <p>Copy</p>
                </TooltipContent>
              </Tooltip>
            </TooltipProvider>
          </div>
          <div className="">
            <span className="block dark:hidden [&>pre]:p-0 [&>pre]:m-0">
              <SyntaxHighlighter
                customStyle={customStyle}
                language={syntax}
                className={cn("syntax-highlighter")}
                showLineNumbers={true}
                wrapLines={true}
                style={materialLight}
              >
                {content}
              </SyntaxHighlighter>
            </span>
            <span className="hidden dark:block [&>pre]:p-0 [&>pre]:m-0">
              <SyntaxHighlighter
                customStyle={customStyle}
                language={syntax}
                className={cn("syntax-highlighter")}
                showLineNumbers={true}
                wrapLines={true}
                style={materialDark}
              >
                {content}
              </SyntaxHighlighter>
            </span>
          </div>
        </div>
    )
}
export default ReadPasteBody;