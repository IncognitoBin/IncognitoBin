export interface CreatePasteRequest {
    title: string;        
    content: string;      
    signature: string;      
    syntax: string;       
    expire: number;         
    burn: boolean;
}