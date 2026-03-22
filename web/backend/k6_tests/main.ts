import { sleep } from 'k6';
import { Options } from 'k6/options';
import { defaultThresholds } from './config';
import { register } from './scenarios/register';
import { login } from './scenarios/login';
import { getCurrentUser } from './scenarios/me';
import { recovery } from './scenarios/recovery';
import { sendCode } from './scenarios/verification';

export const options: Options = {
  thresholds: defaultThresholds,
  scenarios: {
    max_rps: {
      executor: 'ramping-arrival-rate',
      startRate: 10,
      timeUnit: '1s',
      preAllocatedVUs: 50,
      maxVUs: 300,
      stages: [
        { target: 50,  duration: '30s' },
        { target: 150, duration: '60s' },
        { target: 300, duration: '60s' },
        { target: 500, duration: '60s' },
        { target: 0,   duration: '20s' },
      ],
    },
  },
};

export default function (): void {
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
