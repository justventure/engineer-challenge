import http from "k6/http";
import { check } from "k6";
import { headers, BASE_URL } from "../config";

export function login(): Record<string, any> {
  const res = http.post(
    `${BASE_URL}/auth/login`,
    JSON.stringify({ identifier: "test@example.com", password: "Test1234!" }),
    { headers },
  );
  check(res, { "login 2xx": (r) => r.status >= 200 && r.status < 300 });
  return res.cookies;
}
