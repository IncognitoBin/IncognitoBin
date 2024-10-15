export interface GetPaste {
    title: string;        
    content: string;      
    syntax: string;       
    expire: Date;         
    password: boolean;    
    views: number;        
    createdAt?: Date;     
}