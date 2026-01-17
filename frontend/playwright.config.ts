import { defineConfig, devices } from '@playwright/test';

export default defineConfig({
  testDir: './tests',
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  workers: process.env.CI ? 1 : undefined,
  reporter: 'html',
  use: {
    baseURL: 'http://localhost:4321/VeedurIA-Ciudadana', // Astro dev server with base path
    trace: 'on-first-retry',
  },
  projects: [
    {
      name: 'chromium',
      use: { ...devices['Desktop Chrome'] },
    },
  ],
  webServer: {
    command: 'npm run preview', // Run preview for production-like environment
    url: 'http://localhost:4321/VeedurIA-Ciudadana', // Wait for this URL
    reuseExistingServer: true, // Use the server I just started
    timeout: 120 * 1000,
  },
});
