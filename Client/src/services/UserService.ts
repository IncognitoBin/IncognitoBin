import axios from 'axios';
import { UserLoginRequest } from '../models/User/Request/UserLoginRequest';
import { NewUserResponse } from '../models/User/Response/NewUserResponse';
import { UserLoginResponse } from '../models/User/Response/UserLoginResponse';

const API_BASE_URL = import.meta.env.VITE_API_BASE_URL + "/api/user";

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
  static setUserToken(value: string): void {
    const date = new Date();
    date.setTime(date.getTime() + (24 * 60 * 60 * 1000));
    const expires = `expires=${date.toUTCString()}`;
    document.cookie = `token=${value};${expires};path=/`;
  }
  static getUserCookie(): string | undefined {
    var b = document.cookie.match("(^|;)\\s*token\\s*=\\s*([^;]+)");
    return b ? b.pop() : "";
  }
  
}
