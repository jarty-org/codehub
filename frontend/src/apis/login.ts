import request from "@/utils/request";

export type LoginDTO = {
  account: string;
  password: string;
};

export type SignupDTO = {
  username: string;
  password: string;
  email: string;
};

export type LoginResponse = {
  access_token: string;
  refresh_token: string;
};

export const login = (data: LoginDTO) => {
  return request<LoginResponse>({
    url: "/login",
    method: "POST",
    data,
  });
};

export const signup = (data: SignupDTO) => {
  return request<LoginResponse>({
    url: "/signup",
    method: "POST",
    data,
  });
};
