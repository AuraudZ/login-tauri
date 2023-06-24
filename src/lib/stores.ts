import { writable } from "svelte/store";
export interface User {
  username: string;
  id: number;
  license: string;
  licenseExpiry: string;
}

export interface Response {
  message: string;
  user: User;
  error: string;
}

export const userStore = writable<User>();

export const responseStore = writable<Response>();
