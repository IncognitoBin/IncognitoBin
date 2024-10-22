"use client";

import * as React from "react";
import { Check, ChevronsUpDown } from "lucide-react";

import { cn } from "@/lib/utils";
import { Button } from "@/components/ui/button";
import {
  Command,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
} from "@/components/ui/command";
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "@/components/ui/popover";
import { Label } from "../../ui/label";

const languages = [
  { value: "none", label: "None" },
  { value: "abap", label: "ABAP" },
  { value: "abnf", label: "ABNF" },
  { value: "actionscript", label: "ActionScript" },
  { value: "ada", label: "Ada" },
  { value: "agda", label: "Agda" },
  { value: "al", label: "AL" },
  { value: "antlr4", label: "ANTLR4" },
  { value: "apacheconf", label: "ApacheConf" },
  { value: "apex", label: "Apex" },
  { value: "apl", label: "APL" },
  { value: "applescript", label: "AppleScript" },
  { value: "aql", label: "AQL" },
  { value: "arduino", label: "Arduino" },
  { value: "arff", label: "ARFF" },
  { value: "asciidoc", label: "AsciiDoc" },
  { value: "asm6502", label: "ASM 6502" },
  { value: "asmatmel", label: "ASM Atmel" },
  { value: "aspnet", label: "ASP.NET" },
  { value: "autohotkey", label: "AutoHotkey" },
  { value: "autoit", label: "AutoIt" },
  { value: "avisynth", label: "AviSynth" },
  { value: "avro-idl", label: "Avro IDL" },
  { value: "bash", label: "Bash" },
  { value: "basic", label: "Basic" },
  { value: "batch", label: "Batch" },
  { value: "bbcode", label: "BBCode" },
  { value: "bicep", label: "Bicep" },
  { value: "birb", label: "Birb" },
  { value: "bison", label: "Bison" },
  { value: "bnf", label: "BNF" },
  { value: "brainfuck", label: "Brainfuck" },
  { value: "brightscript", label: "BrightScript" },
  { value: "bro", label: "Bro" },
  { value: "bsl", label: "BSL" },
  { value: "c", label: "C" },
  { value: "cfscript", label: "CFScript" },
  { value: "chaiscript", label: "ChaiScript" },
  { value: "cil", label: "CIL" },
  { value: "clike", label: "C-like" },
  { value: "clojure", label: "Clojure" },
  { value: "cmake", label: "CMake" },
  { value: "cobol", label: "COBOL" },
  { value: "coffeescript", label: "CoffeeScript" },
  { value: "concurnas", label: "Concurnas" },
  { value: "coq", label: "Coq" },
  { value: "cpp", label: "C++" },
  { value: "crystal", label: "Crystal" },
  { value: "csharp", label: "C#" },
  { value: "cshtml", label: "CSHTML" },
  { value: "csp", label: "CSP" },
  { value: "css-extras", label: "CSS Extras" },
  { value: "css", label: "CSS" },
  { value: "csv", label: "CSV" },
  { value: "cypher", label: "Cypher" },
  { value: "d", label: "D" },
  { value: "dart", label: "Dart" },
  { value: "dataweave", label: "DataWeave" },
  { value: "dax", label: "DAX" },
  { value: "dhall", label: "Dhall" },
  { value: "diff", label: "Diff" },
  { value: "django", label: "Django" },
  { value: "dns-zone-file", label: "DNS Zone File" },
  { value: "docker", label: "Docker" },
  { value: "dot", label: "Dot" },
  { value: "ebnf", label: "EBNF" },
  { value: "editorconfig", label: "EditorConfig" },
  { value: "eiffel", label: "Eiffel" },
  { value: "ejs", label: "EJS" },
  { value: "elixir", label: "Elixir" },
  { value: "elm", label: "Elm" },
  { value: "erb", label: "ERB" },
  { value: "erlang", label: "Erlang" },
  { value: "etlua", label: "Etlua" },
  { value: "excel-formula", label: "Excel Formula" },
  { value: "factor", label: "Factor" },
  { value: "false", label: "Falselang" },
  { value: "firestore-security-rules", label: "Firestore Security Rules" },
  { value: "flow", label: "Flow" },
  { value: "fortran", label: "Fortran" },
  { value: "fsharp", label: "F#" },
  { value: "ftl", label: "FTL" },
  { value: "gap", label: "GAP" },
  { value: "gcode", label: "G-code" },
  { value: "gdscript", label: "GDScript" },
  { value: "gedcom", label: "GEDCOM" },
  { value: "gherkin", label: "Gherkin" },
  { value: "git", label: "Git" },
  { value: "glsl", label: "GLSL" },
  { value: "gml", label: "GML" },
  { value: "gn", label: "GN" },
  { value: "go-module", label: "Go Module" },
  { value: "go", label: "Go" },
  { value: "graphql", label: "GraphQL" },
  { value: "groovy", label: "Groovy" },
  { value: "haml", label: "HAML" },
  { value: "handlebars", label: "Handlebars" },
  { value: "haskell", label: "Haskell" },
  { value: "haxe", label: "Haxe" },
  { value: "hcl", label: "HCL" },
  { value: "hlsl", label: "HLSL" },
  { value: "hoon", label: "Hoon" },
  { value: "hpkp", label: "HPKP" },
  { value: "hsts", label: "HSTS" },
  { value: "http", label: "HTTP" },
  { value: "ichigojam", label: "IchigoJam" },
  { value: "icon", label: "Icon" },
  { value: "icu-message-format", label: "ICU Message Format" },
  { value: "idris", label: "Idris" },
  { value: "iecst", label: "IECST" },
  { value: "ignore", label: "Ignore" },
  { value: "inform7", label: "Inform7" },
  { value: "ini", label: "INI" },
  { value: "io", label: "IO" },
  { value: "j", label: "J" },
  { value: "java", label: "Java" },
  { value: "javadoc", label: "Javadoc" },
  { value: "javadoclike", label: "Javadoc-like" },
  { value: "javascript", label: "JavaScript" },
  { value: "javastacktrace", label: "Java Stack Trace" },
  { value: "jexl", label: "JEXL" },
  { value: "jolie", label: "Jolie" },
  { value: "jq", label: "JQ" },
  { value: "js-extras", label: "JS Extras" },
  { value: "js-templates", label: "JS Templates" },
  { value: "jsdoc", label: "JSDoc" },
  { value: "json", label: "JSON" },
  { value: "json5", label: "JSON5" },
  { value: "jsonp", label: "JSONP" },
  { value: "jsstacktrace", label: "JS Stack Trace" },
  { value: "jsx", label: "JSX" },
  { value: "julia", label: "Julia" },
  { value: "keepalived", label: "Keepalived" },
  { value: "keyman", label: "Keyman" },
  { value: "kotlin", label: "Kotlin" },
  { value: "kumir", label: "Kumir" },
  { value: "kusto", label: "Kusto" },
  { value: "latex", label: "LaTeX" },
  { value: "latte", label: "Latte" },
  { value: "less", label: "Less" },
  { value: "lilypond", label: "LilyPond" },
  { value: "liquid", label: "Liquid" },
  { value: "lisp", label: "Lisp" },
  { value: "livescript", label: "LiveScript" },
  { value: "llvm", label: "LLVM" },
  { value: "log", label: "Log" },
  { value: "lolcode", label: "LOLCode" },
  { value: "lua", label: "Lua" },
  { value: "magma", label: "Magma" },
  { value: "makefile", label: "Makefile" },
  { value: "markdown", label: "Markdown" },
  { value: "markup-templating", label: "Markup Templating" },
  { value: "markup", label: "Markup" },
  { value: "matlab", label: "MATLAB" },
  { value: "maxscript", label: "MaxScript" },
  { value: "mel", label: "MEL" },
  { value: "mermaid", label: "Mermaid" },
  { value: "mizar", label: "Mizar" },
  { value: "mongodb", label: "MongoDB" },
  { value: "monkey", label: "Monkey" },
  { value: "moonscript", label: "MoonScript" },
  { value: "n1ql", label: "N1QL" },
  { value: "n4js", label: "N4JS" },
  { value: "nand2tetris-hdl", label: "Nand2Tetris HDL" },
  { value: "naniscript", label: "NaniScript" },
  { value: "nasm", label: "NASM" },
  { value: "neon", label: "Neon" },
  { value: "nevod", label: "Nevod" },
  { value: "nginx", label: "Nginx" },
  { value: "nim", label: "Nim" },
  { value: "nix", label: "Nix" },
  { value: "nsis", label: "NSIS" },
  { value: "objectivec", label: "Objective-C" },
  { value: "ocaml", label: "OCaml" },
  { value: "opencl", label: "OpenCL" },
  { value: "openqasm", label: "OpenQASM" },
  { value: "oz", label: "Oz" },
  { value: "parigp", label: "PARI/GP" },
  { value: "parser", label: "Parser" },
  { value: "pascal", label: "Pascal" },
  { value: "pascaligo", label: "Pascaligo" },
  { value: "pcaxis", label: "PCAxis" },
  { value: "peoplecode", label: "PeopleCode" },
  { value: "perl", label: "Perl" },
  { value: "php-extras", label: "PHP Extras" },
  { value: "php", label: "PHP" },
  { value: "phpdoc", label: "PHPDoc" },
  { value: "plsql", label: "PL/SQL" },
  { value: "powerquery", label: "PowerQuery" },
  { value: "powershell", label: "PowerShell" },
  { value: "processing", label: "Processing" },
  { value: "prolog", label: "Prolog" },
  { value: "promql", label: "PromQL" },
  { value: "properties", label: "Properties" },
  { value: "protobuf", label: "Protobuf" },
  { value: "psl", label: "PSL" },
  { value: "pug", label: "Pug" },
  { value: "puppet", label: "Puppet" },
  { value: "pure", label: "Pure" },
  { value: "purebasic", label: "PureBasic" },
  { value: "purescript", label: "PureScript" },
  { value: "python", label: "Python" },
  { value: "q", label: "Q" },
  { value: "qml", label: "QML" },
  { value: "qore", label: "Qore" },
  { value: "qsharp", label: "Q#" },
  { value: "r", label: "R" },
  { value: "racket", label: "Racket" },
  { value: "reason", label: "Reason" },
  { value: "regex", label: "Regex" },
  { value: "rego", label: "Rego" },
  { value: "renpy", label: "Ren'Py" },
  { value: "rest", label: "reST" },
  { value: "rip", label: "Rip" },
  { value: "roboconf", label: "Roboconf" },
  { value: "robotframework", label: "Robot Framework" },
  { value: "ruby", label: "Ruby" },
  { value: "rust", label: "Rust" },
  { value: "sas", label: "SAS" },
  { value: "sass", label: "SASS" },
  { value: "scala", label: "Scala" },
  { value: "scheme", label: "Scheme" },
  { value: "scss", label: "SCSS" },
  { value: "shell-session", label: "Shell Session" },
  { value: "smali", label: "Smali" },
  { value: "smalltalk", label: "Smalltalk" },
  { value: "smarty", label: "Smarty" },
  { value: "sml", label: "SML" },
  { value: "solidity", label: "Solidity" },
  { value: "solution-file", label: "Solution File" },
  { value: "soy", label: "Soy" },
  { value: "sparql", label: "SPARQL" },
  { value: "splunk-spl", label: "Splunk SPL" },
  { value: "sqf", label: "SQF" },
  { value: "sql", label: "SQL" },
  { value: "squirrel", label: "Squirrel" },
  { value: "stan", label: "Stan" },
  { value: "stylus", label: "Stylus" },
  { value: "swift", label: "Swift" },
  { value: "systemd", label: "Systemd" },
  { value: "t4-cs", label: "T4 CS" },
  { value: "t4-templating", label: "T4 Templating" },
  { value: "t4-vb", label: "T4 VB" },
  { value: "tap", label: "TAP" },
  { value: "tcl", label: "TCL" },
  { value: "textile", label: "Textile" },
  { value: "toml", label: "TOML" },
  { value: "tremor", label: "Tremor" },
  { value: "tsx", label: "TSX" },
  { value: "tt2", label: "TT2" },
  { value: "turtle", label: "Turtle" },
  { value: "twig", label: "Twig" },
  { value: "typescript", label: "TypeScript" },
  { value: "typoscript", label: "TypoScript" },
  { value: "unrealscript", label: "UnrealScript" },
  { value: "uorazor", label: "UO Razor" },
  { value: "uri", label: "URI" },
  { value: "v", label: "V" },
  { value: "vala", label: "Vala" },
  { value: "vbnet", label: "VB.NET" },
  { value: "velocity", label: "Velocity" },
  { value: "verilog", label: "Verilog" },
  { value: "vhdl", label: "VHDL" },
  { value: "vim", label: "Vim" },
  { value: "visual-basic", label: "Visual Basic" },
  { value: "warpscript", label: "WarpScript" },
  { value: "wasm", label: "WebAssembly" },
  { value: "web-idl", label: "Web IDL" },
  { value: "wiki", label: "Wiki" },
  { value: "wolfram", label: "Wolfram" },
  { value: "wren", label: "Wren" },
  { value: "xeora", label: "Xeora" },
  { value: "xml-doc", label: "XML Doc" },
  { value: "xojo", label: "Xojo" },
  { value: "xquery", label: "XQuery" },
  { value: "yaml", label: "YAML" },
  { value: "yang", label: "Yang" },
  { value: "zig", label: "Zig" },
];
interface BurnAfterReadSwitchProps {
    value: string;
    setValue: (event: string) => void;
}
export const SyntaxBox: React.FC<BurnAfterReadSwitchProps> = ({ value, setValue }) =>{
  const [open, setOpen] = React.useState(false);

  return (
    <div className="flex flex-col">
      <Label htmlFor="combobox" className="mb-1.5">Syntax Highlighting</Label>
      <div id="combobox">
        <Popover open={open} onOpenChange={setOpen}>
          <PopoverTrigger asChild>
            <Button
              variant="outline"
              role="combobox"
              aria-expanded={open}
              className="w-[220px] justify-between"
            >
              {value
                ? languages.find((lang) => lang.value === value)?.label
                : "Select Language"}
              <ChevronsUpDown className="ml-2 h-4 w-4 shrink-0 opacity-50" />
            </Button>
          </PopoverTrigger>
          <PopoverContent className="w-[220px] p-0">
            <Command>
              <CommandInput placeholder="Search" />
              <CommandList>
                <CommandEmpty>Not found.</CommandEmpty>
                <CommandGroup className="cursor-pointer">
                  {languages.map((lang) => (
                    <CommandItem
                      key={lang.value}
                      value={lang.value}
                      onSelect={(currentValue) => {
                        setValue(
                          currentValue === value ? "none" : currentValue
                        );
                        setOpen(false);
                      }}
                    >
                      <Check
                        className={cn(
                          "mr-2 h-4 w-4",
                          value === lang.value ? "opacity-100" : "opacity-0"
                        )}
                      />
                      {lang.label}
                    </CommandItem>
                  ))}
                </CommandGroup>
              </CommandList>
            </Command>
          </PopoverContent>
        </Popover>
      </div>
    </div>
  );
}
