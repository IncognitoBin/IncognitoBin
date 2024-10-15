import axios from 'axios';
import { UserLoginRequest } from '../models/User/Request/UserLoginRequest';
import { NewUserResponse } from '../models/User/Response/NewUserResponse';
import { UserLoginResponse } from '../models/User/Response/UserLoginResponse';


// Replace this URL with your actual backend API URL
const API_BASE_URL = 'http://localhost:8181/api/user';

export class UserService {
  static async New(): Promise<NewUserResponse> {
    try {
      const response = await axios.get<NewUserResponse>(`${API_BASE_URL}`);
      return response.data;
    } catch (error) {
      console.error(`Error getting User ID:`, error);
      throw error;
    }
  }
  static async Login(UserLogin: UserLoginRequest): Promise<UserLoginResponse> {
    try {
      const response = await axios.post<UserLoginResponse>(API_BASE_URL, UserLogin);
      return response.data;
    } catch (error) {
      console.error('Error login:', error);
      throw error;
    }
  }
  static async LogOut(userToken: string): Promise<void> {
    try {
      await axios.delete(API_BASE_URL, {
        headers: {
          Authorization: userToken,
        },
      });
    } catch (error) {
      console.error(`Error logout`, error);
      throw error;
    }
  }
}
