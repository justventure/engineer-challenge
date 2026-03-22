import http from "k6/http";
import { check } from "k6";
import { BASE_URL } from "../config";

export function getCurrentUser(cookies: Record<string, any>): void {
  const res = http.get(`${BASE_URL}/auth/me`, { cookies });
  check(res, { "me 200": (r) => r.status === 200 });
}
