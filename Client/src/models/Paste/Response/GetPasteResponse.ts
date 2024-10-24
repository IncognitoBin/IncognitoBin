export interface GetPaste {
    title: string;        
    content: string;      
    signature: string;      
    syntax: string;       
    expire: number;         
    views: number;        
    createdAt?: Date;     
}