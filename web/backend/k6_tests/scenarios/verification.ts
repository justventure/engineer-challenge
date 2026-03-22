import http from 'k6/http';
import { check } from 'k6';
import { headers, BASE_URL } from '../config';

export function sendCode(): void {
  const res = http.post(
    `${BASE_URL}/auth/verify/code/send`,
    JSON.stringify({ email: 'test@example.com' }),
    { headers }
  );
  check(res, { 'send_code 2xx': (r) => r.status < 300 });
}

export function submitCode(code = '123456'): void {
  const res = http.post(
    `${BASE_URL}/auth/verify/code/submit`,
    JSON.stringify({ code }),
    { headers }
  );
  check(res, { 'submit_code 2xx': (r) => r.status < 300 });
}
