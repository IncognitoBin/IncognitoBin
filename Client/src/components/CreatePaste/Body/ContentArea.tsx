
import { cn } from "@/lib/utils";
import { useEffect, useRef } from "react";
import { Prism as SyntaxHighlighter } from "react-syntax-highlighter";
import {
  materialDark,
  materialLight,
} from "react-syntax-highlighter/dist/cjs/styles/prism";
interface ContentAreaProps {
    Syntax: string;
    Content: string;
    setContent: (event: string) => void;
    tap: number;
  }
const ContentArea: React.FC<ContentAreaProps> = ({
    Syntax,
    Content,
    setContent,
    tap,
  }) => {
    const textareaRef = useRef<HTMLTextAreaElement>(null);
    useEffect(() => {
      if (textareaRef.current) {
        textareaRef.current.style.height = "auto"; 
        textareaRef.current.style.height = `${textareaRef.current.scrollHeight}px`; 
      }
    }, [Content]);
    const customStyle = {
        marginTop: "-2px",
        marginBottom: "0px",
        padding: "10px 0px 10px 5px",
        borderRadius: "0 0 10px 10px",
        border: "0px solid gray",
      };
    return(
        <>
        <textarea
              ref={textareaRef}
              value={Content}
              onChange={(e)=>{setContent(e.target.value)}}
              rows={1}
              placeholder="Your Content Here..."
              className={cn(
                "w-full overflow-hidden resize-none bg-transparent p-5 min-h-60 border-none outline-none",
                {
                      hidden: tap !== 0,
                    }
                    )}
                    />
            <div
              className={cn("w-full", {
                  hidden: tap !== 1,
                })}
                >
              <span className="block dark:hidden [&>pre]:p-0 [&>pre]:m-0">
                <SyntaxHighlighter
                  customStyle={customStyle}
                  language={Syntax}
                  className={cn("min-h-60")}
                  showLineNumbers={true}
                  wrapLines={true}
                  style={materialLight}
                  >
                  {Content || "No Content"}
                </SyntaxHighlighter>
              </span>
              <span className="hidden dark:block [&>pre]:p-0 [&>pre]:m-0">
                <SyntaxHighlighter
                  customStyle={customStyle}
                  language={Syntax}
                  className={cn("min-h-60")}
                  showLineNumbers={true}
                  wrapLines={true}
                  style={materialDark}
                  >
                  {Content || "No Content"}
                </SyntaxHighlighter>
              </span>
            </div>
           </>
    )
}

export default ContentArea;