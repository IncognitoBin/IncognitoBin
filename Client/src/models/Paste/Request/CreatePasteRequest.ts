export interface CreatePaste {
    title: string;        
    content: string;      
    syntax: string;       
    expire: number;         
    password: boolean;
    burn: boolean;
}