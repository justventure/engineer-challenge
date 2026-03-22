import http from 'k6/http';
import { check } from 'k6';
import { headers, BASE_URL } from '../config';

export function recovery(): void {
  const res = http.post(
    `${BASE_URL}/auth/recovery`,
    JSON.stringify({ email: 'test@example.com' }),
    { headers }
  );
  check(res, { 'recovery 2xx': (r) => r.status < 300 });
}
