// main.ts
import { sleep } from "k6";

// config.ts
var BASE_URL = __ENV.BASE_URL || "http://localhost:8080/api/v1";
var headers = { "Content-Type": "application/json" };
var defaultThresholds = {
  http_req_duration: ["p(95)<500", "p(99)<1000"],
  http_req_failed: ["rate<0.01"]
};

// scenarios/register.ts
import http from "k6/http";
import { check } from "k6";
function register() {
  const uid = `u_${Date.now()}_${Math.random().toString(36).slice(2, 7)}`;
  const res = http.post(
    `${BASE_URL}/auth/register`,
    JSON.stringify({
      identifier: `${uid}@test.com`,
      username: uid,
      password: "Test1234!",
      geo_location: "ES"
    }),
    { headers }
  );
  check(res, {
    "register 2xx": (r) => r.status >= 200 && r.status < 300,
    "register <500ms": (r) => r.timings.duration < 500
  });
}

// scenarios/login.ts
import http2 from "k6/http";
import { check as check2 } from "k6";
function login() {
  const res = http2.post(
    `${BASE_URL}/auth/login`,
    JSON.stringify({ identifier: "test@example.com", password: "Test1234!" }),
    { headers }
  );
  check2(res, { "login 2xx": (r) => r.status >= 200 && r.status < 300 });
  return res.cookies;
}

// scenarios/me.ts
import http3 from "k6/http";
import { check as check3 } from "k6";
function getCurrentUser(cookies) {
  const res = http3.get(`${BASE_URL}/auth/me`, { cookies });
  check3(res, { "me 200": (r) => r.status === 200 });
}

// scenarios/recovery.ts
import http4 from "k6/http";
import { check as check4 } from "k6";
function recovery() {
  const res = http4.post(
    `${BASE_URL}/auth/recovery`,
    JSON.stringify({ email: "test@example.com" }),
    { headers }
  );
  check4(res, { "recovery 2xx": (r) => r.status < 300 });
}

// scenarios/verification.ts
import http5 from "k6/http";
import { check as check5 } from "k6";
function sendCode() {
  const res = http5.post(
    `${BASE_URL}/auth/verify/code/send`,
    JSON.stringify({ email: "test@example.com" }),
    { headers }
  );
  check5(res, { "send_code 2xx": (r) => r.status < 300 });
}

// main.ts
var options = {
  thresholds: defaultThresholds,
  scenarios: {
    max_rps: {
      executor: "ramping-arrival-rate",
      startRate: 10,
      timeUnit: "1s",
      preAllocatedVUs: 50,
      maxVUs: 300,
      stages: [
        { target: 50, duration: "30s" },
        { target: 150, duration: "60s" },
        { target: 300, duration: "60s" },
        { target: 500, duration: "60s" },
        { target: 0, duration: "20s" }
      ]
    }
  }
};
function main_default() {
  const roll = Math.random();
  if (roll < 0.55) {
    const cookies = login();
    getCurrentUser(cookies);
  } else if (roll < 0.75) {
    register();
  } else if (roll < 0.88) {
    recovery();
  } else {
    sendCode();
  }
  sleep(0.1);
}
export {
  main_default as default,
  options
};
