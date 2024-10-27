import axios from 'axios';
import { GetPaste } from '../models/Paste/Response/GetPasteResponse';
import { CreatePasteRequest } from '../models/Paste/Request/CreatePasteRequest';
import { CreatedPaste } from '../models/Paste/Response/CreatedPasteResponse';
import { GetPasteGenInfo } from '@/models/Paste/Response/GetPasteGenInfoResponse';

const API_BASE_URL = import.meta.env.VITE_API_BASE_URL + "/api/paste";

export class PasteService {

  static async getAllPastes(userToken: string): Promise<GetPasteGenInfo[]> {
    try {
      const response = await axios.get<GetPasteGenInfo[]>(API_BASE_URL, {
        headers: {
          Authorization: userToken,
        },
      });
      return response.data;
    } catch (error) {
      console.error('Error fetching pastes:', error);
      throw error;
    }
  }
  static async getPasteById(id: string): Promise<GetPaste> {
    try {
      const response = await axios.get<GetPaste>(`${API_BASE_URL}/${id}`);
      return response.data;
    } catch (error) {
      console.error(`Error fetching paste with ID ${id}:`, error);
      throw error;
    }
  }
  static async createPaste(newPaste: CreatePasteRequest,userToken:string): Promise<CreatedPaste> {
    try {
      const response = await axios.post<CreatedPaste>(API_BASE_URL, newPaste, {
        headers: {
          Authorization: userToken,
        },
      });
      return response.data;
    } catch (error) {
      console.error('Error creating paste:', error);
      throw error;
    }
  }
  static async deletePaste(id: string,userToken: string): Promise<void> {
    try {
      await axios.delete(`${API_BASE_URL}/${id}`, {
        headers: {
          Authorization: userToken,
        },
      });
    } catch (error) {
      console.error(`Error deleting paste with ID ${id}:`, error);
      throw error;
    }
  }
}
