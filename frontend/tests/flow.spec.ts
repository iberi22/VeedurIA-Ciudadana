import { test, expect } from '@playwright/test';

test.describe('Veeduría Ciudadana Flow', () => {
  test('should navigate from dashboard to report form with pre-filled data', async ({ page }) => {
    // 1. Visit Homepage
    await page.goto('/');
    await page.waitForLoadState('domcontentloaded');

    // Check for main content visibility
    await expect(page.locator('main')).toBeVisible();

    // 2. Wait for Red Flags to load (Contract Cards)
    // Wait for loading to disappear
    await expect(page.locator('text=Cargando información...')).not.toBeVisible({ timeout: 15000 }).catch(() => {});

    // Check for explicit error message
    const errorMsg = page.locator('text=No se pudieron cargar los datos');
    if (await errorMsg.isVisible()) {
        console.log('Error UI visible');
        throw new Error('Data loading failed in UI');
    }

    // We expect at least one contract card to be present after loading
    const card = page.locator('.group').first();
    await expect(card).toBeVisible({ timeout: 30000 });

    // Get text from the card to verify later
    const contractText = await card.innerText();

    // 3. Click the card to navigate
    await card.click();

    // 4. Verify Navigation to /denunciar
    await expect(page).toHaveURL(/.*\/denunciar/);

    // 5. Verify Form is pre-filled
    // The ID field should not be empty
    const idInput = page.locator('input[placeholder="Ej: CO1.PCCNTR.123456"]');
    await expect(idInput).not.toBeEmpty();

    // 6. Verify Buttons are active (since ID is present)
    const emailBtn = page.locator('button:has-text("Enviar Email")');
    await expect(emailBtn).toBeEnabled();
  });
});
