export interface GetPaste {
    title: string;        
    content: string;      
    signature: string;      
    syntax: string;       
    expire: Date;         
    views: number;        
    createdAt?: Date;     
}